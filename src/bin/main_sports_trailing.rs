// Trailing bot for sports, politics, and other binary/3-way markets: trade a single market by slug. Supports 2-outcome (binary) and 3-outcome (e.g. Team A / Team B / Draw). First leg: trail and buy only tokens under 0.5. Second leg: trail and buy opposite (binary) or one of the two remaining (3-way). Option: once per market or continuous.

use anyhow::{Context, Result};
use clap::Parser;
use polymarket_trading_bot::config::{Args, Config};
use log::warn;
use std::sync::Arc;
use chrono::{DateTime, NaiveDateTime, FixedOffset, TimeZone};
use rust_decimal::prelude::ToPrimitive;

use polymarket_trading_bot::api::PolymarketApi;
use polymarket_trading_bot::detector::{BuyOpportunity, TokenType};
use polymarket_trading_bot::trader::Trader;

const MIN_FIRST_BUY_COST: f64 = 1.0;

fn format_remaining_hms(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{}h {:02}m {:02}s", h, m, s)
    } else if m > 0 {
        format!("{}m {:02}s", m, s)
    } else {
        format!("{}s", s)
    }
}

fn bid_f64(token: &polymarket_trading_bot::models::TokenPrice) -> f64 {
    token
        .bid
        .as_ref()
        .and_then(|d| d.to_f64())
        .unwrap_or(0.0)
}

fn ask_f64(token: &polymarket_trading_bot::models::TokenPrice) -> f64 {
    token
        .ask
        .as_ref()
        .and_then(|d| d.to_f64())
        .or_else(|| token.bid.as_ref().and_then(|d| d.to_f64()))
        .unwrap_or(0.0)
}

fn first_buy_units_and_investment(base_shares: f64, price: f64) -> (f64, f64) {
    let min_units = (MIN_FIRST_BUY_COST / price).max(base_shares);
    let units = (min_units * 100.0).ceil() / 100.0;
    let investment = units * price;
    (units, investment)
}

/// US Eastern (EDT) offset: UTC-4 (seconds east of UTC).
const EASTERN_OFFSET_SECS: i32 = -4 * 3600;

/// Parse ISO 8601 end date to Unix timestamp. Returns None if unparseable.
/// - If the string has timezone (Z, +00:00, -05:00, etc.), it is used as-is.
/// - If the string has no timezone (e.g. "2026-03-05T19:00:00"), it is treated as US Eastern
///   (sports/politics often use Eastern). This avoids "market ended" when the API sends
///   game time in Eastern and we would otherwise interpret it as UTC.
fn parse_end_date_iso(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.timestamp() as u64);
    }
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.3f",
    ];
    for fmt in &formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(s, fmt) {
            let offset = FixedOffset::east_opt(EASTERN_OFFSET_SECS)?;
            let dt = offset.from_local_datetime(&naive).single()?;
            return Some(dt.timestamp() as u64);
        }
    }
    None
}

async fn fetch_token_price(
    api: &PolymarketApi,
    token_id: &str,
) -> Option<polymarket_trading_bot::models::TokenPrice> {
    let bid = api.get_price(token_id, "BUY").await.ok();
    let ask = api.get_price(token_id, "SELL").await.ok();
    if bid.is_some() || ask.is_some() {
        Some(polymarket_trading_bot::models::TokenPrice {
            token_id: token_id.to_string(),
            bid,
            ask,
        })
    } else {
        None
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum SportsTrailingState {
    /// Binary: trail both tokens; buy one that satisfies ask >= lowest + trailing_stop (under 0.5).
    WaitingFirst {
        low0: f64,
        high0: f64,
        low1: f64,
        high1: f64,
    },
    /// 3-way: trail all three tokens; buy one under 0.5 when it triggers.
    WaitingFirst3 {
        low0: f64,
        high0: f64,
        low1: f64,
        high1: f64,
        low2: f64,
        high2: f64,
    },
    /// First buy in flight (skip updates until resolved).
    FirstBuyPending {
        first_is_token0: bool,
        first_price: f64,
        shares: f64,
        opp_lowest: f64,
        revert_low0: f64,
        revert_high0: f64,
        revert_low1: f64,
        revert_high1: f64,
    },
    /// 3-way first buy in flight.
    FirstBuyPending3 {
        first_index: usize,
        first_price: f64,
        shares: f64,
        revert_low0: f64,
        revert_high0: f64,
        revert_low1: f64,
        revert_high1: f64,
        revert_low2: f64,
        revert_high2: f64,
    },
    /// Binary: first token bought; trail opposite.
    FirstBought {
        first_is_token0: bool,
        first_price: f64,
        shares: f64,
        opposite_lowest: f64,
    },
    /// 3-way: first token bought; trail the two remaining (rem1 = (first+1)%3, rem2 = (first+2)%3).
    FirstBought3 {
        first_index: usize,
        first_price: f64,
        shares: f64,
        rem1_low: f64,
        rem1_high: f64,
        rem2_low: f64,
        rem2_high: f64,
    },
    /// Both legs bought for this round. If continuous, will reset to WaitingFirst.
    Done,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();
    let config = Config::load(&args.config)?;
    let is_simulation = args.is_simulation();

    let slug = config
        .trading
        .slug
        .as_ref()
        .filter(|s| !s.is_empty())
        .context("Config must set trading.slug (e.g. sports or politics market slug)")?;

    let continuous = config.trading.continuous;
    let trailing_stop = config.trading.trailing_stop_point.unwrap_or(0.03);
    let shares = config
        .trading
        .trailing_shares
        .unwrap_or_else(|| config.trading.fixed_trade_amount / 0.5);
    let check_interval_ms = config.trading.check_interval_ms;

    eprintln!("🚀 Sports & Politics Trailing Bot — slug: {}", slug);
    eprintln!(
        "Mode: {} | Continuous: {}",
        if is_simulation { "SIMULATION" } else { "LIVE" },
        continuous
    );
    eprintln!(
        "Trailing stop: {:.4} | Shares per side: {} | Check interval: {} ms",
        trailing_stop, shares, check_interval_ms
    );

    let api = Arc::new(PolymarketApi::new(
        config.polymarket.gamma_api_url.clone(),
        config.polymarket.clob_api_url.clone(),
        config.polymarket.api_key.clone(),
        config.polymarket.api_secret.clone(),
        config.polymarket.api_passphrase.clone(),
        config.polymarket.private_key.clone(),
        config.polymarket.proxy_wallet_address.clone(),
        config.polymarket.signature_type,
    ));

    eprintln!("\n═══════════════════════════════════════════════════════════");
    eprintln!("🔐 Authenticating...");
    eprintln!("═══════════════════════════════════════════════════════════");
    api.authenticate().await.context("Authentication failed")?;
    eprintln!("✅ Authentication successful!\n");

    let market = api.get_market_by_slug(slug).await.context("Failed to load market by slug")?;
    let condition_id = market.condition_id.clone();
    let details = api
        .get_market(&condition_id)
        .await
        .context("Failed to get market details (tokens, end time)")?;

    let num_tokens = details.tokens.len();
    if num_tokens < 2 || num_tokens > 3 {
        anyhow::bail!("Market must have 2 or 3 outcome tokens, got {}", num_tokens);
    }
    // Prefer outcome names from Gamma (e.g. "Over 220.5" / "Under 220.5") when present; else use CLOB token labels ("Over"/"Under")
    let outcome_names: Vec<String> = if let Some(ref outcomes_json) = market.outcomes {
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(outcomes_json) {
            if parsed.len() == num_tokens && parsed.iter().all(|s| !s.is_empty()) {
                parsed
            } else {
                details.tokens.iter().map(|t| t.outcome.clone()).collect()
            }
        } else {
            details.tokens.iter().map(|t| t.outcome.clone()).collect()
        }
    } else {
        details.tokens.iter().map(|t| t.outcome.clone()).collect()
    };
    let out0 = outcome_names[0].as_str();
    let out1 = outcome_names[1].as_str();
    let (token2_id, out2) = if num_tokens >= 3 {
        (Some(details.tokens[2].token_id.clone()), outcome_names[2].as_str())
    } else {
        (None, "")
    };
    let token0_id = details.tokens[0].token_id.clone();
    let token1_id = details.tokens[1].token_id.clone();

    let end_ts = parse_end_date_iso(&details.end_date_iso)
        .or_else(|| market.end_date_iso.as_deref().and_then(parse_end_date_iso))
        .or_else(|| market.end_date_iso_alt.as_deref().and_then(parse_end_date_iso));

    eprintln!("Market: {} | Condition: {}...", market.question, &condition_id[..condition_id.len().min(24)]);
    eprintln!("Tokens: {} | Token0 ({}): {}...", num_tokens, out0, &token0_id[..token0_id.len().min(20)]);
    eprintln!("Token1 ({}): {}...", out1, &token1_id[..token1_id.len().min(20)]);
    if let (Some(ref tid), o2) = (&token2_id, out2) {
        eprintln!("Token2 ({}): {}...", o2, &tid[..tid.len().min(20)]);
    }
    eprintln!("API end_date_iso: {}", details.end_date_iso);
    if let Some(et) = end_ts {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let remaining = et.saturating_sub(now);
        eprintln!("Parsed end (Unix): {} | Time remaining: {}s", et, remaining);
    }
    eprintln!("Exit only when API reports market closed.");

    let trader = Arc::new(Trader::new(api.clone(), config.trading.clone(), is_simulation, None)?);
    let state: Arc<tokio::sync::Mutex<SportsTrailingState>> = Arc::new(tokio::sync::Mutex::new(
        if num_tokens == 3 {
            SportsTrailingState::WaitingFirst3 {
                low0: 1.0,
                high0: 0.0,
                low1: 1.0,
                high1: 0.0,
                low2: 1.0,
                high2: 0.0,
            }
        } else {
            SportsTrailingState::WaitingFirst {
                low0: 1.0,
                high0: 0.0,
                low1: 1.0,
                high1: 0.0,
            }
        },
    ));

    let period_timestamp = end_ts.unwrap_or(0).saturating_sub(3600); // placeholder for logging

    let mut iterations_since_closed_check: u64 = 0;
    const CLOSED_CHECK_INTERVAL: u64 = 60;
    let mut price_log_counter: u64 = 0;
    const PRICE_LOG_EVERY: u64 = 1;
    let mut first_price_logged = false;

    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let time_remaining_seconds = end_ts.map(|et| et.saturating_sub(now)).unwrap_or(999_999);

        iterations_since_closed_check += 1;
        let should_check_closed = time_remaining_seconds == 0 || iterations_since_closed_check >= CLOSED_CHECK_INTERVAL;
        if should_check_closed {
            iterations_since_closed_check = 0;
            match api.get_market(&condition_id).await {
                Ok(refresh) if refresh.closed => {
                    eprintln!("Market closed (API). Exiting.");
                    break;
                }
                Ok(_) if time_remaining_seconds == 0 => {
                    eprintln!("Past end_date_iso but market still open (API); continuing to trade.");
                }
                Ok(_) => {}
                Err(e) => {
                    warn!("Could not refresh market status: {}. Continuing.", e);
                }
            }
        }

        let effective_time_remaining = if time_remaining_seconds == 0 {
            60
        } else {
            time_remaining_seconds
        };

        let (p0, p1, p2_opt) = if num_tokens == 3 {
            let tid2 = token2_id.as_ref().unwrap();
            let (price0, price1, price2) = tokio::join!(
                fetch_token_price(api.as_ref(), &token0_id),
                fetch_token_price(api.as_ref(), &token1_id),
                fetch_token_price(api.as_ref(), tid2),
            );
            match (price0, price1, price2) {
                (Some(a), Some(b), Some(c)) => (a, b, Some(c)),
                _ => {
                    eprintln!("Price fetch missed (no data for one or more tokens); retrying...");
                    tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                    continue;
                }
            }
        } else {
            let (price0, price1) = tokio::join!(
                fetch_token_price(api.as_ref(), &token0_id),
                fetch_token_price(api.as_ref(), &token1_id),
            );
            let (Some(p0), Some(p1)) = (price0, price1) else {
                eprintln!("Price fetch missed (no data for one or both tokens); retrying...");
                tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                continue;
            };
            (p0, p1, None)
        };

        let ask0 = ask_f64(&p0);
        let ask1 = ask_f64(&p1);
        let ask2 = p2_opt.as_ref().map(ask_f64).unwrap_or(0.0);

        let bid0 = bid_f64(&p0);
        let bid1 = bid_f64(&p1);
        let bid2 = p2_opt.as_ref().map(bid_f64).unwrap_or(0.0);
        if !first_price_logged {
            first_price_logged = true;
            if num_tokens == 3 {
                eprintln!("Price fetch OK | {} {:.4}/{:.4} | {} {:.4}/{:.4} | {} {:.4}/{:.4}", out0, bid0, ask0, out1, bid1, ask1, out2, bid2, ask2);
            } else {
                eprintln!("Price fetch OK | {} {:.4} / {:.4} | {} {:.4} / {:.4}", out0, bid0, ask0, out1, bid1, ask1);
            }
        }
        price_log_counter += 1;
        if price_log_counter >= PRICE_LOG_EVERY {
            price_log_counter = 0;
            let guard = state.lock().await;
            if num_tokens == 3 {
                let (l0, l1, l2, t0, t1, t2) = match &*guard {
                    SportsTrailingState::WaitingFirst3 { low0, low1, low2, .. } => (
                        *low0, *low1, *low2,
                        *low0 + trailing_stop, *low1 + trailing_stop, *low2 + trailing_stop,
                    ),
                    SportsTrailingState::FirstBought3 { rem1_low, rem2_low, .. } => (
                        *rem1_low, *rem2_low, 0.0,
                        rem1_low + trailing_stop, rem2_low + trailing_stop, 0.0,
                    ),
                    _ => (ask0, ask1, ask2, ask0 + trailing_stop, ask1 + trailing_stop, ask2 + trailing_stop),
                };
                eprintln!(
                    "Prices | {} {:.4}/{:.4} {} {:.4}/{:.4} {} {:.4}/{:.4} | low/tr: {:.4}/{:.4} {:.4}/{:.4} {:.4}/{:.4} | remaining={}",
                    out0, bid0, ask0, out1, bid1, ask1, out2, bid2, ask2, l0, t0, l1, t1, l2, t2, format_remaining_hms(effective_time_remaining)
                );
            } else {
                let (low0, low1, trig0, trig1) = match &*guard {
                    SportsTrailingState::WaitingFirst { low0, low1, .. } => (*low0, *low1, *low0 + trailing_stop, *low1 + trailing_stop),
                    SportsTrailingState::FirstBought { opposite_lowest, .. } => (*opposite_lowest, *opposite_lowest, opposite_lowest + trailing_stop, opposite_lowest + trailing_stop),
                    _ => (ask0, ask1, ask0 + trailing_stop, ask1 + trailing_stop),
                };
                eprintln!(
                    "Prices | {} {:.4} / {:.4} {} {:.4} / {:.4} | low/trigger: {:.4}/{:.4} {:.4}/{:.4} | remaining={}",
                    out0, bid0, ask0, out1, bid1, ask1, low0, trig0, low1, trig1, format_remaining_hms(effective_time_remaining)
                );
            }
        }

        {
            let guard = state.lock().await;
            if matches!(&*guard, SportsTrailingState::FirstBuyPending { .. } | SportsTrailingState::FirstBuyPending3 { .. }) {
                drop(guard);
                tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                continue;
            }
        }

        let mut guard = state.lock().await;
        match &mut *guard {
            SportsTrailingState::WaitingFirst {
                low0,
                high0,
                low1,
                high1,
            } => {
                let old_high0 = *high0;
                let old_high1 = *high1;
                *low0 = (*low0).min(ask0);
                *high0 = (*high0).max(ask0);
                *low1 = (*low1).min(ask1);
                *high1 = (*high1).max(ask1);

                let trigger0 = *low0 + trailing_stop;
                let trigger1 = *low1 + trailing_stop;

                if ask0 > old_high0 {
                    *low0 = ask0;
                }
                if ask1 > old_high1 {
                    *low1 = ask1;
                }

                let buy0 = ask0 >= trigger0 && ask0 <= old_high0;
                let buy1 = ask1 >= trigger1 && ask1 <= old_high1;

                // First leg: only trail and buy the token whose price is under 0.5 (underdog).
                let token0_under_half = ask0 < 0.5;
                let token1_under_half = ask1 < 0.5;

                let do_buy0 = buy0 && token0_under_half;
                let do_buy1 = buy1 && token1_under_half;

                // If both under 0.5 and both trigger, buy the cheaper one.
                let (buy_first_0, price) = if do_buy0 && (!do_buy1 || ask0 <= ask1) {
                    (true, ask0)
                } else if do_buy1 {
                    (false, ask1)
                } else {
                    (false, 0.0)
                };

                if do_buy0 || do_buy1 {
                    drop(guard);
                    execute_first_buy(
                        state.clone(),
                        trader.clone(),
                        buy_first_0,
                        price,
                        shares,
                        &p0,
                        &p1,
                        &condition_id,
                        period_timestamp,
                        effective_time_remaining,
                        out0,
                        out1,
                    )
                    .await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                    continue;
                }
            }
            SportsTrailingState::WaitingFirst3 {
                low0,
                high0,
                low1,
                high1,
                low2,
                high2,
            } => {
                let old_high0 = *high0;
                let old_high1 = *high1;
                let old_high2 = *high2;
                *low0 = (*low0).min(ask0);
                *high0 = (*high0).max(ask0);
                *low1 = (*low1).min(ask1);
                *high1 = (*high1).max(ask1);
                *low2 = (*low2).min(ask2);
                *high2 = (*high2).max(ask2);

                let trigger0 = *low0 + trailing_stop;
                let trigger1 = *low1 + trailing_stop;
                let trigger2 = *low2 + trailing_stop;

                if ask0 > old_high0 {
                    *low0 = ask0;
                }
                if ask1 > old_high1 {
                    *low1 = ask1;
                }
                if ask2 > old_high2 {
                    *low2 = ask2;
                }

                let buy0 = ask0 >= trigger0 && ask0 <= old_high0;
                let buy1 = ask1 >= trigger1 && ask1 <= old_high1;
                let buy2 = ask2 >= trigger2 && ask2 <= old_high2;

                let token0_under = ask0 < 0.5;
                let token1_under = ask1 < 0.5;
                let token2_under = ask2 < 0.5;

                let do_buy0 = buy0 && token0_under;
                let do_buy1 = buy1 && token1_under;
                let do_buy2 = buy2 && token2_under;

                // Pick the cheapest triggering underdog.
                let (first_index, price) = if do_buy0 && (!do_buy1 || ask0 <= ask1) && (!do_buy2 || ask0 <= ask2) {
                    (0usize, ask0)
                } else if do_buy1 && (!do_buy2 || ask1 <= ask2) {
                    (1, ask1)
                } else if do_buy2 {
                    (2, ask2)
                } else {
                    (0, 0.0)
                };

                if do_buy0 || do_buy1 || do_buy2 {
                    let p2 = p2_opt.as_ref().unwrap();
                    drop(guard);
                    execute_first_buy_3(
                        state.clone(),
                        trader.clone(),
                        first_index,
                        price,
                        shares,
                        &p0,
                        &p1,
                        p2,
                        ask0,
                        ask1,
                        ask2,
                        &condition_id,
                        period_timestamp,
                        effective_time_remaining,
                        out0,
                        out1,
                        out2,
                    )
                    .await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                    continue;
                }
            }
            SportsTrailingState::FirstBought {
                first_is_token0,
                first_price,
                shares: first_shares,
                opposite_lowest,
            } => {
                let (opp_ask, _opp_token, opp_id, is_first_side) = if *first_is_token0 {
                    (ask1, &p1, &token1_id, false)
                } else {
                    (ask0, &p0, &token0_id, true)
                };
                *opposite_lowest = (*opposite_lowest).min(opp_ask);
                let trigger_at = *opposite_lowest + trailing_stop;
                if opp_ask >= trigger_at {
                    let _first_price_val = *first_price;
                    let first_shares_val = *first_shares;
                    let _first_is_0 = *first_is_token0;
                    drop(guard);
                    let investment = first_shares_val * opp_ask;
                    let opp = BuyOpportunity {
                        condition_id: condition_id.clone(),
                        token_id: opp_id.clone(),
                        token_type: if is_first_side {
                            TokenType::BtcUp
                        } else {
                            TokenType::BtcDown
                        },
                        bid_price: opp_ask,
                        period_timestamp,
                        time_remaining_seconds: effective_time_remaining,
                        time_elapsed_seconds: 0,
                        use_market_order: true,
                        investment_amount_override: Some(investment),
                        is_individual_hedge: false,
                        is_standard_hedge: false,
                        dual_limit_shares: Some(first_shares_val),
                    };
                    if let Err(e) = trader.execute_buy(&opp).await {
                                warn!("Trailing second buy failed: {}", e);
                    } else {
                        polymarket_trading_bot::log_println!(
                            "📈 Trailing second buy: {} at ${:.4} x {:.6}",
                            if is_first_side { out0 } else { out1 },
                            opp_ask,
                            first_shares_val
                        );
                        let mut g = state.lock().await;
                        if continuous {
                            *g = SportsTrailingState::WaitingFirst {
                                low0: 1.0,
                                high0: 0.0,
                                low1: 1.0,
                                high1: 0.0,
                            };
                        } else {
                            *g = SportsTrailingState::Done;
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                    continue;
                }
            }
            SportsTrailingState::FirstBought3 {
                first_index,
                first_price: _,
                shares: first_shares,
                rem1_low,
                rem1_high,
                rem2_low,
                rem2_high,
            } => {
                let rem1_idx = (*first_index + 1) % 3;
                let rem2_idx = (*first_index + 2) % 3;
                let asks = [ask0, ask1, ask2];
                let rem1_ask = asks[rem1_idx];
                let rem2_ask = asks[rem2_idx];
                let old_rem1_high = *rem1_high;
                let old_rem2_high = *rem2_high;

                *rem1_low = (*rem1_low).min(rem1_ask);
                *rem1_high = (*rem1_high).max(rem1_ask);
                *rem2_low = (*rem2_low).min(rem2_ask);
                *rem2_high = (*rem2_high).max(rem2_ask);

                if rem1_ask > old_rem1_high {
                    *rem1_low = rem1_ask;
                }
                if rem2_ask > old_rem2_high {
                    *rem2_low = rem2_ask;
                }

                let trigger1 = *rem1_low + trailing_stop;
                let trigger2 = *rem2_low + trailing_stop;
                let buy_rem1 = rem1_ask >= trigger1 && rem1_ask <= old_rem1_high;
                let buy_rem2 = rem2_ask >= trigger2 && rem2_ask <= old_rem2_high;

                if !buy_rem1 && !buy_rem2 {
                    drop(guard);
                    tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                    continue;
                }

                let token_ids: [&str; 3] = [&token0_id, &token1_id, token2_id.as_ref().unwrap()];
                let outcomes = [out0, out1, out2];
                let (buy_which, opp_ask, opp_id, opp_outcome): (usize, f64, &str, &str) = if buy_rem1 && (!buy_rem2 || rem1_ask <= rem2_ask) {
                    (rem1_idx, rem1_ask, token_ids[rem1_idx], outcomes[rem1_idx])
                } else {
                    (rem2_idx, rem2_ask, token_ids[rem2_idx], outcomes[rem2_idx])
                };

                let first_shares_val = *first_shares;
                drop(guard);
                let investment = first_shares_val * opp_ask;
                let opp = BuyOpportunity {
                    condition_id: condition_id.clone(),
                    token_id: opp_id.to_string(),
                    token_type: if buy_which == 0 {
                        TokenType::BtcUp
                    } else {
                        TokenType::BtcDown
                    },
                    bid_price: opp_ask,
                    period_timestamp,
                    time_remaining_seconds: effective_time_remaining,
                    time_elapsed_seconds: 0,
                    use_market_order: true,
                    investment_amount_override: Some(investment),
                    is_individual_hedge: false,
                    is_standard_hedge: false,
                    dual_limit_shares: Some(first_shares_val),
                };
                if let Err(e) = trader.execute_buy(&opp).await {
                    warn!("Trailing second buy (3-way) failed: {}", e);
                } else {
                    polymarket_trading_bot::log_println!(
                        "📈 Trailing second buy: {} at ${:.4} x {:.6}",
                        opp_outcome,
                        opp_ask,
                        first_shares_val
                    );
                    let mut g = state.lock().await;
                    if continuous {
                        *g = SportsTrailingState::WaitingFirst3 {
                            low0: 1.0,
                            high0: 0.0,
                            low1: 1.0,
                            high1: 0.0,
                            low2: 1.0,
                            high2: 0.0,
                        };
                    } else {
                        *g = SportsTrailingState::Done;
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                continue;
            }
            SportsTrailingState::Done => {
                if !continuous {
                    drop(guard);
                    tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
                    continue;
                }
                if num_tokens == 3 {
                    *guard = SportsTrailingState::WaitingFirst3 {
                        low0: ask0,
                        high0: ask0,
                        low1: ask1,
                        high1: ask1,
                        low2: ask2,
                        high2: ask2,
                    };
                } else {
                    *guard = SportsTrailingState::WaitingFirst {
                        low0: ask0,
                        high0: ask0,
                        low1: ask1,
                        high1: ask1,
                    };
                }
            }
            SportsTrailingState::FirstBuyPending { .. } | SportsTrailingState::FirstBuyPending3 { .. } => {}
        }
        drop(guard);
        tokio::time::sleep(tokio::time::Duration::from_millis(check_interval_ms)).await;
    }

    Ok(())
}

async fn execute_first_buy(
    state: Arc<tokio::sync::Mutex<SportsTrailingState>>,
    trader: Arc<Trader>,
    first_is_token0: bool,
    buy_price: f64,
    base_shares: f64,
    p0: &polymarket_trading_bot::models::TokenPrice,
    p1: &polymarket_trading_bot::models::TokenPrice,
    condition_id: &str,
    period_timestamp: u64,
    time_remaining_seconds: u64,
    out0: &str,
    out1: &str,
) {
    let (units, investment) = first_buy_units_and_investment(base_shares, buy_price);
    let (token_id, token_type, opp_ask) = if first_is_token0 {
        (p0.token_id.clone(), TokenType::BtcUp, ask_f64(p1))
    } else {
        (p1.token_id.clone(), TokenType::BtcDown, ask_f64(p0))
    };

    let revert_low0 = ask_f64(p0).min(1.0);
    let revert_high0 = ask_f64(p0).max(0.0);
    let revert_low1 = ask_f64(p1).min(1.0);
    let revert_high1 = ask_f64(p1).max(0.0);

    {
        let mut g = state.lock().await;
        *g = SportsTrailingState::FirstBuyPending {
            first_is_token0,
            first_price: buy_price,
            shares: units,
            opp_lowest: opp_ask,
            revert_low0,
            revert_high0,
            revert_low1,
            revert_high1,
        };
    }

    let opp = BuyOpportunity {
        condition_id: condition_id.to_string(),
        token_id: token_id.clone(),
        token_type,
        bid_price: buy_price,
        period_timestamp,
        time_remaining_seconds,
        time_elapsed_seconds: 0,
        use_market_order: true,
        investment_amount_override: Some(investment),
        is_individual_hedge: false,
        is_standard_hedge: false,
        dual_limit_shares: Some(units),
    };

    let result = trader.execute_buy(&opp).await;
    let mut g = state.lock().await;
    match result {
        Err(e) => {
            warn!("Trailing first buy failed: {}", e);
            *g = SportsTrailingState::WaitingFirst {
                low0: revert_low0,
                high0: revert_high0,
                low1: revert_low1,
                high1: revert_high1,
            };
        }
        Ok(()) => {
            polymarket_trading_bot::log_println!(
                                "📈 Trailing first buy: {} at ${:.4} x {:.6} (cost ${:.2})",
                if first_is_token0 { out0 } else { out1 },
                buy_price,
                units,
                investment
            );
            *g = SportsTrailingState::FirstBought {
                first_is_token0,
                first_price: buy_price,
                shares: units,
                opposite_lowest: opp_ask,
            };
        }
    }
}

async fn execute_first_buy_3(
    state: Arc<tokio::sync::Mutex<SportsTrailingState>>,
    trader: Arc<Trader>,
    first_index: usize,
    buy_price: f64,
    base_shares: f64,
    p0: &polymarket_trading_bot::models::TokenPrice,
    p1: &polymarket_trading_bot::models::TokenPrice,
    p2: &polymarket_trading_bot::models::TokenPrice,
    ask0: f64,
    ask1: f64,
    ask2: f64,
    condition_id: &str,
    period_timestamp: u64,
    time_remaining_seconds: u64,
    out0: &str,
    out1: &str,
    out2: &str,
) {
    let (units, investment) = first_buy_units_and_investment(base_shares, buy_price);
    let tokens = [p0, p1, p2];
    let outcomes = [out0, out1, out2];
    let token_id = tokens[first_index].token_id.clone();
    let token_type = if first_index == 0 {
        TokenType::BtcUp
    } else {
        TokenType::BtcDown
    };

    let revert_low0 = ask0.min(1.0);
    let revert_high0 = ask0.max(0.0);
    let revert_low1 = ask1.min(1.0);
    let revert_high1 = ask1.max(0.0);
    let revert_low2 = ask2.min(1.0);
    let revert_high2 = ask2.max(0.0);

    let rem1_idx = (first_index + 1) % 3;
    let rem2_idx = (first_index + 2) % 3;
    let asks = [ask0, ask1, ask2];
    let rem1_ask = asks[rem1_idx];
    let rem2_ask = asks[rem2_idx];

    {
        let mut g = state.lock().await;
        *g = SportsTrailingState::FirstBuyPending3 {
            first_index,
            first_price: buy_price,
            shares: units,
            revert_low0,
            revert_high0,
            revert_low1,
            revert_high1,
            revert_low2,
            revert_high2,
        };
    }

    let opp = BuyOpportunity {
        condition_id: condition_id.to_string(),
        token_id: token_id.clone(),
        token_type,
        bid_price: buy_price,
        period_timestamp,
        time_remaining_seconds,
        time_elapsed_seconds: 0,
        use_market_order: true,
        investment_amount_override: Some(investment),
        is_individual_hedge: false,
        is_standard_hedge: false,
        dual_limit_shares: Some(units),
    };

    let result = trader.execute_buy(&opp).await;
    let mut g = state.lock().await;
    match result {
        Err(e) => {
            warn!("Trailing first buy (3-way) failed: {}", e);
            *g = SportsTrailingState::WaitingFirst3 {
                low0: revert_low0,
                high0: revert_high0,
                low1: revert_low1,
                high1: revert_high1,
                low2: revert_low2,
                high2: revert_high2,
            };
        }
        Ok(()) => {
            polymarket_trading_bot::log_println!(
                "📈 Trailing first buy: {} at ${:.4} x {:.6} (cost ${:.2})",
                outcomes[first_index],
                buy_price,
                units,
                investment
            );
            *g = SportsTrailingState::FirstBought3 {
                first_index,
                first_price: buy_price,
                shares: units,
                rem1_low: rem1_ask,
                rem1_high: rem1_ask,
                rem2_low: rem2_ask,
                rem2_high: rem2_ask,
            };
        }
    }
}
