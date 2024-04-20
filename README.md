# Trade Rust - Advanced Trading Bot System

## Overview
This project is designed to be a highly modular and scalable trading bot system built in Rust, capable of running multiple bots simultaneously, each with its unique trading strategy. The system supports real-time trading, testing, and backtesting functionalities. It is structured to keep sensitive data and proprietary strategies private while sharing the core infrastructure publicly.

## Project Structure

```plaintext
trading_system/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── Makefile
├── .gitignore
└── src/
    ├── main.rs
    └── data_manager
        └── mod.rs
```

## Getting Started
### Prerequisites
- Rust `1.77.2`
- Node `20.6.0`

---
If you need to incorporate your own datasets into our system, you can easily add your files to the **`data/`** folder. The files should follow a specific naming convention and format to ensure compatibility and proper functionality.

### File Naming Format
`data/<currency_pair>-tick-<start_date>-<end_date>.csv`

### File Content Format
```
timestamp,askPrice,bidPrice
1704146457222,1.27331,1.27184
1704146641141,1.27351,1.27155
1704146643447,1.27351,1.27191
...
```
Tick data provides a high-resolution view of market movements by capturing every price change, offering significant advantages for trading analysis. This granularity is crucial for high-frequency trading strategies and precise backtesting, allowing traders to simulate and evaluate their strategies under real-world conditions. Moreover, tick data provides deeper insights into market sentiment and volatility, revealing subtle trends and opportunities that broader summaries like candlesticks might miss. This detailed data helps traders enhance their risk management by adapting to even minute market fluctuations, potentially increasing profitability and reducing losses.

## Run a backtest
You can run 
```bash
make backtest gbpusd 2024-02-01 2024-02-02
```
### format :
`make backtest <market_epic> <start_date> <end_date>`

## Other
If you need help run : 
```bash
make help
```
If you need help on Makefile, you can run 
```bash
make mhelp
```