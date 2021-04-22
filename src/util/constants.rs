use super::uberstate::UberIdentifier;

pub const DEFAULT_SPAWN: &str = "MarshSpawn.Main";
pub const MOKI_SPAWNS: &[&str] = &[
    "MarshSpawn.Main",
    "HowlsDen.Teleporter",
    "GladesTown.Teleporter",
    "InnerWellspring.Teleporter",
    "MidnightBurrows.Teleporter",
];
pub const GORLEK_SPAWNS: &[&str] = &[
    "MarshSpawn.Main",
    "HowlsDen.Teleporter",
    "EastHollow.Teleporter",
    "GladesTown.Teleporter",
    "InnerWellspring.Teleporter",
    "MidnightBurrows.Teleporter",
    "WoodsEntry.Teleporter",
    "WoodsMain.Teleporter",
    "LowerReach.Teleporter",
    "UpperDepths.Teleporter",
    "EastPools.Teleporter",
    "LowerWastes.WestTP",
    "LowerWastes.EastTP",
    "UpperWastes.NorthTP",
    "WillowsEnd.InnerTP",
];
pub const RELIC_ZONES: &[&str] = &[
    "Inkwater Marsh",
    "Midnight Burrows",
    "Kwoloks Hollow",
    "Wellspring Glades",
    "The Wellspring",
    "Luma Pools",
    "Silent Woods",
    "Baurs Reach",
    "Mouldwood Depths",
    "Windswept Wastes",
    "Willows End",
];
pub const KEYSTONE_DOORS: &[(&str, u8)] = &[
    ("MarshSpawn.KeystoneDoor", 2),
    ("HowlsDen.KeystoneDoor", 2),
    ("MarshPastOpher.EyestoneDoor", 2),
    ("MidnightBurrows.KeystoneDoor", 4),
    ("WoodsEntry.KeystoneDoor", 2),
    ("WoodsMain.KeystoneDoor", 4),
    ("LowerReach.KeystoneDoor", 4),
    ("UpperReach.KeystoneDoor", 4),
    ("UpperDepths.EntryKeystoneDoor", 2),
    ("UpperDepths.CentralKeystoneDoor", 2),
    ("UpperPools.KeystoneDoor", 4),
    ("UpperWastes.KeystoneDoor", 2),
];
pub const RESERVE_SLOTS: usize = 2;  // how many slots to reserve after ranodm placements for the next iteration
pub const RETRIES: u16 = 5;  // How many retries to allow when generating a seed
pub const RANDOM_PROGRESSION: f64 = 0.1;  // How likely to choose a progression item as random placement
pub const SHOP_PRICES: &[(&str, UberIdentifier, UberIdentifier)] = &[
    (
        "TwillenShop.Overcharge",
        UberIdentifier {
            uber_group: 2,
            uber_id: 1,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 0,
        }
    ),
    (
        "TwillenShop.TripleJump",
        UberIdentifier {
            uber_group: 2,
            uber_id: 2,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 1,
        }
    ),
    (
        "TwillenShop.Wingclip",
        UberIdentifier {
            uber_group: 2,
            uber_id: 3,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 2,
        }
    ),
    (
        "TwillenShop.Swap",
        UberIdentifier {
            uber_group: 2,
            uber_id: 5,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 3,
        }
    ),
    (
        "TwillenShop.LightHarvest",
        UberIdentifier {
            uber_group: 2,
            uber_id: 19,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 4,
        }
    ),
    (
        "TwillenShop.Vitality",
        UberIdentifier {
            uber_group: 2,
            uber_id: 22,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 5,
        }
    ),
    (
        "TwillenShop.Energy",
        UberIdentifier {
            uber_group: 2,
            uber_id: 26,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 6,
        }
    ),
    (
        "TwillenShop.Finesse",
        UberIdentifier {
            uber_group: 2,
            uber_id: 40,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 7,
        }
    ),
    (
        "OpherShop.WaterBreath",
        UberIdentifier {
            uber_group: 1,
            uber_id: 23,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 8,
        }
    ),
    (
        "OpherShop.Spike",
        UberIdentifier {
            uber_group: 1,
            uber_id: 74,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 9,
        }
    ),
    (
        "OpherShop.SpiritSmash",
        UberIdentifier {
            uber_group: 1,
            uber_id: 98,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 10,
        }
    ),
    (
        "OpherShop.Teleport",
        UberIdentifier {
            uber_group: 1,
            uber_id: 105,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 11,
        }
    ),
    (
        "OpherShop.SpiritStar",
        UberIdentifier {
            uber_group: 1,
            uber_id: 106,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 12,
        }
    ),
    (
        "OpherShop.Blaze",
        UberIdentifier {
            uber_group: 1,
            uber_id: 115,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 13,
        }
    ),
    (
        "OpherShop.Sentry",
        UberIdentifier {
            uber_group: 1,
            uber_id: 116,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 14,
        }
    ),
    (
        "LupoShop.HCMapIcon",
        UberIdentifier {
            uber_group: 48248,
            uber_id: 19396,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 15,
        }
    ),
    (
        "LupoShop.ECMapIcon",
        UberIdentifier {
            uber_group: 48248,
            uber_id: 57987,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 16,
        }
    ),
    (
        "LupoShop.ShardMapIcon",
        UberIdentifier {
            uber_group: 48248,
            uber_id: 41666,
        },
        UberIdentifier {
            uber_group: 12,
            uber_id: 17,
        }
    ),
];