# Artifacts Game Rust API Client

## Introduction
A Rust API client for Artifacts, providing type-safe access to the game's HTTP endpoints. This client simplifies automation and tool development for the Artifacts MMORPG.

## Overview

```rust
    let tokens = "TOKEN";
    let api = Api::new(tokens.to_string());
    let result = api.status().await;
    assert!(result.is_ok());
```


## API Implementation Status

## Status
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get Status | GET | `/` | ✅ |

## My Account
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get My Account | GET | `/my/account` | ⬜ |
| Get Bank Details | GET | `/bank/details` | ⬜ |
| Get Bank Items | GET | `/bank/items` | ✅ |
| Get GE Sell Orders | GET | `/ge/orders` | ⬜ |
| Get GE Sell History | GET | `/ge/history` | ⬜ |
| Get Account Details | GET | `/account/details` | ⬜ |
| Change Password | POST | `/account/password` | ⬜ |

## My Characters
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Action Move | POST | `/action/move` | ✅ |
| Action Rest | POST | `/action/rest` | ✅ |
| Action Equip Item | POST | `/action/equip` | ✅ |
| Action Unequip Item | POST | `/action/unequip` | ✅ |
| Action Use Item | POST | `/action/use` | ✅ |
| Action Fight | POST | `/action/fight` | ✅ |
| Action Gathering | POST | `/action/gathering` | ✅ |
| Action Crafting | POST | `/action/crafting` | ✅ |
| Action Deposit Bank Gold | POST | `/action/bank/deposit/gold` | ✅ |
| Action Deposit Bank | POST | `/action/bank/deposit` | ✅ |
| Action Withdraw Bank | POST | `/action/bank/withdraw` | ✅ |
| Action Withdraw Bank Gold | POST | `/action/bank/withdraw/gold` | ✅ |
| Action Buy Bank Expansion | POST | `/action/bank/expand` | ⬜ |
| Action Recycling | POST | `/action/recycling` | ✅ |
| Action GE Buy Item | POST | `/action/ge/buy` | ⬜ |
| Action GE Create Sell Order | POST | `/action/ge/sell` | ⬜ |
| Action GE Cancel Sell Order | POST | `/action/ge/cancel` | ⬜ |
| Action Complete Task | POST | `/action/task/complete` | ⬜ |
| Action Task Exchange | POST | `/action/task/exchange` | ⬜ |
| Action Accept New Task | POST | `/action/task/accept` | ⬜ |
| Action Task Trade | POST | `/action/task/trade` | ⬜ |
| Action Task Cancel | POST | `/action/task/cancel` | ⬜ |
| Action Christmas Exchange | POST | `/action/christmas/exchange` | ⬜ |
| Action Delete Item | POST | `/action/item/delete` | ⬜ |
| Get All Characters Logs | GET | `/characters/logs` | ⬜ |
| Get My Characters | GET | `/my/characters` | ⬜ |

## Accounts
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Create Account | POST | `/accounts` | ⬜ |
| Get Account Achievements | GET | `/accounts/achievements` | ⬜ |
| Get Account | GET | `/accounts/:id` | ⬜ |

## Achievements
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Achievements | GET | `/achievements` | ⬜ |
| Get Achievement | GET | `/achievements/:id` | ⬜ |

## Badges
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Badges | GET | `/badges` | ⬜ |
| Get Badge | GET | `/badges/:id` | ⬜ |

## Characters
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Create Character | POST | `/characters` | ⬜ |
| Delete Character | POST | `/characters/:id` | ⬜ |
| Get Character | GET | `/characters/:id` | ✅ |

## Events
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Active Events | GET | `/events/active` | ⬜ |
| Get All Events | GET | `/events` | ⬜ |

## Grand Exchange
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get GE Sell History | GET | `/ge/history` | ⬜ |
| Get GE Sell Orders | GET | `/ge/orders` | ⬜ |
| Get GE Sell Order | GET | `/ge/orders/:id` | ⬜ |

## Items
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Items | GET | `/items` | ✅ |
| Get Item | GET | `/items/:id` | ✅ |

## Leaderboard
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get Characters Leaderboard | GET | `/leaderboard/characters` | ⬜ |
| Get Accounts Leaderboard | GET | `/leaderboard/accounts` | ⬜ |

## Maps
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Maps | GET | `/maps` | ✅ |
| Get Map | GET | `/maps/:id` | ✅ |

## Monsters
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Monsters | GET | `/monsters` | ✅ |
| Get Monster | GET | `/monsters/:id` | ✅ |

## Resources
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Resources | GET | `/resources` | ✅ |
| Get Resource | GET | `/resources/:id` | ✅ |

## Tasks
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Get All Tasks | GET | `/tasks` | ⬜ |
| Get Task | GET | `/tasks/:id` | ⬜ |
| Get All Tasks Rewards | GET | `/tasks/rewards` | ⬜ |
| Get Tasks Reward | GET | `/tasks/rewards/:id` | ⬜ |

## Token
| Endpoint | Method | Path | Status |
|----------|--------|------|--------|
| Generate Token | POST | `/token` | ⬜ |

Legend:
- ⬜ Not Implemented
- ✅ Implemented
- 🚧 In Progress