IF !(SELECT * FROM class:warrior) {
    CREATE class:warrior CONTENT {
        name: "Warrior",
        skills: [{ name: "Punch", damage_upper: 10, damage_lower: 20 }]
    };
};
IF !(SELECT * FROM class:mage) {
    CREATE class:mage CONTENT {
        name: "Mage",
        skills: [{ name: "Fireball", damage_upper: 10, damage_lower: 20 }]
    };
};
IF !(SELECT * FROM class:ranger) {
    CREATE class:ranger CONTENT {
        name: "Ranger",
        skills: [{ name: "Nature's Fury", damage_upper: 10, damage_lower: 20 }]
    };
};
IF !(SELECT * FROM class:monk) {
    CREATE class:monk CONTENT {
        name: "Monk",
        skills: [{ name: "Holy Nova", damage_upper: 10, damage_lower: 20 }]
    };
};
IF !(SELECT * FROM item:leather_cap) {
    CREATE item:leather_cap CONTENT {
        name: "Leather Cap",
        slot: "Head",
        rarity: "Common",
    };
};
IF !(SELECT * FROM item:iron_helm) {
    CREATE item:iron_helm CONTENT {
        name: "Iron Helm",
        slot: "Head",
        rarity: "Rare",
    };
};
IF !(SELECT * FROM item:dragons_visage) {
    CREATE item:dragons_visage CONTENT {
        name: "Dragon's Visage",
        slot: "Head",
        rarity: "Epic",
    };
};
IF !(SELECT * FROM item:crown_of_the_eternal) {
    CREATE item:crown_of_the_eternal CONTENT {
        name: "Crown of the Eternal",
        slot: "Head",
        rarity: "Legendary",
    };
};
IF !(SELECT * FROM item:cloth_tunic) {
    CREATE item:cloth_tunic CONTENT {
        name: "Cloth Tunic",
        slot: "Chest",
        rarity: "Common",
    };
};
IF !(SELECT * FROM item:chainmail_armor) {
    CREATE item:chainmail_armor CONTENT {
        name: "Chainmail Armor",
        slot: "Chest",
        rarity: "Rare",
    };
};
IF !(SELECT * FROM item:phoenix_feather_cuirass) {
    CREATE item:phoenix_feather_cuirass CONTENT {
        name: "Phoenix Feather Cuirass",
        slot: "Chest",
        rarity: "Epic",
    };
};
IF !(SELECT * FROM item:heartguard_of_the_titans) {
    CREATE item:heartguard_of_the_titans CONTENT {
        name: "Heartguard of the Titans",
        slot: "Chest",
        rarity: "Legendary",
    };
};    
IF !(SELECT * FROM item:woolen_gloves) {
    CREATE item:woolen_gloves CONTENT {
        name: "Woolen Gloves",
        slot: "Hands",
        rarity: "Common",
    };
};
IF !(SELECT * FROM item:plated_gauntlets) {
    CREATE item:plated_gauntlets CONTENT {
        name: "Plated Gauntlets",
        slot: "Hands",
        rarity: "Rare",
    };
};
IF !(SELECT * FROM item:grasp_of_the_wyrm) {
    CREATE item:grasp_of_the_wyrm CONTENT {
        name: "Grasp of the Wyrm",
        slot: "Hands",
        rarity: "Epic",
    };
};
IF !(SELECT * FROM item:fists_of_the_astral_walker) {
    CREATE item:fists_of_the_astral_walker CONTENT {
        name: "Fists of the Astral Walker",
        slot: "Hands",
        rarity: "Legendary",
    };
};
IF !(SELECT * FROM item:simple_pants) {
    CREATE item:simple_pants CONTENT {
        name: "Simple Pants",
        slot: "Legs",
        rarity: "Common",
    };
};
IF !(SELECT * FROM item:greaves_of_fortitude) {
    CREATE item:greaves_of_fortitude CONTENT {
        name: "Greaves of Fortitude",
        slot: "Legs",
        rarity: "Rare",
    };
};
IF !(SELECT * FROM item:leggings_of_the_archmage) {
    CREATE item:leggings_of_the_archmage CONTENT {
        name: "Leggings of the Archmage",
        slot: "Legs",
        rarity: "Epic",
    };
};
IF !(SELECT * FROM item:protectors_greaves) {
    CREATE item:protectors_greaves CONTENT {
        name: "Protector's Greaves",
        slot: "Legs",
        rarity: "Legendary",
    };
};
IF !(SELECT * FROM item:sandals_of_comfort) {
    CREATE item:sandals_of_comfort CONTENT {
        name: "Sandals of Comfort",
        slot: "Feet",
        rarity: "Common",
    };
};
IF !(SELECT * FROM item:steel_plated_boots) {
    CREATE item:steel_plated_boots CONTENT {
        name: "Steel-Plated Boots",
        slot: "Feet",
        rarity: "Rare",
    };
};
IF !(SELECT * FROM item:windstrider_boots) {
    CREATE item:windstrider_boots CONTENT {
        name: "Windstrider Boots",
        slot: "Feet",
        rarity: "Epic",
    };
};
IF !(SELECT * FROM item:steps_of_the_timeless_one) {
    CREATE item:steps_of_the_timeless_one CONTENT {
        name: "Steps of the Timeless One",
        slot: "Feet",
        rarity: "Legendary",
    };
};
IF !(SELECT * FROM item:wooden_sword) {
    CREATE item:wooden_sword CONTENT {
        name: "Wooden Sword",
        slot: "MainHand",
        rarity: "Common",
        weapon_skill: {
            name: "Basic Slash",
            damage_upper: 5,
            damage_lower: 10,
        }
    };
};
IF !(SELECT * FROM item:steel_longsword) {
    CREATE item:steel_longsword CONTENT {
        name: "Steel Longsword",
        slot: "MainHand",
        rarity: "Rare",
        weapon_skill: {
            name: "Strong Slash",
            damage_upper: 10,
            damage_lower: 20,
        }
    };
};
IF !(SELECT * FROM item:flamesoul_saber) {
    CREATE item:flamesoul_saber CONTENT {
        name: "Flamesoul Saber",
        slot: "MainHand",
        rarity: "Epic",
        weapon_skill: {
            name: "Flaming Strike",
            damage_upper: 20,
            damage_lower: 40,
        }
    };
};
IF !(SELECT * FROM item:excaliburs_echo) {
    CREATE item:excaliburs_echo CONTENT {
        name: "Excalibur's Echo",
        slot: "MainHand",
        rarity: "Legendary",
        weapon_skill: {
            name: "Divine Wrath",
            damage_upper: 40,
            damage_lower: 80,
        }
    };
};
IF !(SELECT * FROM item:wooden_shield) {
    CREATE item:wooden_shield CONTENT {
        name: "Wooden Shield",
        slot: "OffHand",
        rarity: "Common",
        weapon_skill: {
            name: "Wooden Block",
            damage_upper: 1,
            damage_lower: 2,
        }
    };
};
IF !(SELECT * FROM item:kite_shield) {
    CREATE item:kite_shield CONTENT {
        name: "Kite Shield",
        slot: "OffHand",
        rarity: "Rare",
        weapon_skill: {
            name: "Steel Parry",
            damage_upper: 2,
            damage_lower: 4,
        }
    };
};
IF !(SELECT * FROM item:aegis_of_arcana) {
    CREATE item:aegis_of_arcana CONTENT {
        name: "Aegis of Arcana",
        slot: "OffHand",
        rarity: "Epic",
        weapon_skill: {
            name: "Arcane Shield",
            damage_upper: 4,
            damage_lower: 8,
        }
    };
};
IF !(SELECT * FROM item:dawnbreaker_shield) {
    CREATE item:dawnbreaker_shield CONTENT {
        name: "Dawnbreaker Shield",
        slot: "OffHand",
        rarity: "Legendary",
        weapon_skill: {
            name: "Dawnbreaker Defense",
            damage_upper: 8,
            damage_lower: 16,
        }
    };
};