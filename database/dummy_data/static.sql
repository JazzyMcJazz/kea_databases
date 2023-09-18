-- Classes
INSERT INTO class (name) VALUES ('Warrior');
INSERT INTO class (name) VALUES ('Mage');
INSERT INTO class (name) VALUES ('Ranger');
INSERT INTO class (name) VALUES ('Monk');

-- Class Skills
INSERT INTO class_skill (name, damage_upper, damage_lower, class_id) VALUES ('Punch', 10, 20, 1);
INSERT INTO class_skill (name, damage_upper, damage_lower, class_id) VALUES ('Fireball', 10, 20, 2);
INSERT INTO class_skill (name, damage_upper, damage_lower, class_id) VALUES ('Nature\'s Fury', 10, 20, 3);
INSERT INTO class_skill (name, damage_upper, damage_lower, class_id) VALUES ('Holy Nova', 10, 20, 4);

-- Head Gear
INSERT INTO item (name, slot, rarity) VALUES ('Leather Cap', 'Head', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Iron Helm', 'Head', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Dragon\'s Visage', 'Head', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Crown of the Eternal', 'Head', 'Legendary');

-- Chest Gear
INSERT INTO item (name, slot, rarity) VALUES ('Cloth Tunic', 'Chest', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Chainmail Armor', 'Chest', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Phoenix Feather Cuirass', 'Chest', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Heartguard of the Titans', 'Chest', 'Legendary');

-- Hand Gear
INSERT INTO item (name, slot, rarity) VALUES ('Woolen Gloves', 'Hands', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Plated Gauntlets', 'Hands', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Grasp of the Wyrm', 'Hands', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Fists of the Astral Walker', 'Hands', 'Legendary');

-- Leg Gear
INSERT INTO item (name, slot, rarity) VALUES ('Simple Pants', 'Legs', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Greaves of Fortitude', 'Legs', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Leggings of the Archmage', 'Legs', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Protector\'s Greaves', 'Legs', 'Legendary');

-- Feet Gear
INSERT INTO item (name, slot, rarity) VALUES ('Sandals of Comfort', 'Feet', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Steel-Plated Boots', 'Feet', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Windstrider Boots', 'Feet', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Steps of the Timeless One', 'Feet', 'Legendary');

-- MainHand Gear
INSERT INTO item (name, slot, rarity) VALUES ('Wooden Sword', 'MainHand', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Steel Longsword', 'MainHand', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Flamesoul Saber', 'MainHand', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Excalibur\'s Echo', 'MainHand', 'Legendary');

-- OffHand Gear
INSERT INTO item (name, slot, rarity) VALUES ('Wooden Shield', 'OffHand', 'Common');
INSERT INTO item (name, slot, rarity) VALUES ('Kite Shield', 'OffHand', 'Rare');
INSERT INTO item (name, slot, rarity) VALUES ('Aegis of Arcana', 'OffHand', 'Epic');
INSERT INTO item (name, slot, rarity) VALUES ('Dawnbreaker Shield', 'OffHand', 'Legendary');

-- Weapon Skills: MainHand Gear (item_ids 21 to 24)
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Basic Slash', 5, 10, 21);
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Strong Slash', 10, 20, 22);
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Flaming Strike', 20, 40, 23);
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Divine Wrath', 40, 80, 24);

-- Weapon Skills: OffHand Gear (item_ids 25 to 28)
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Wooden Block', 1, 2, 25);
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Steel Parry', 2, 4, 26);
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Arcane Shield', 4, 8, 27);
INSERT INTO weapon_skill (name, damage_lower, damage_upper, item_id) VALUES ('Dawnbreaker Defense', 8, 16, 28);