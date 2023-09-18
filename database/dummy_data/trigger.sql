
DELIMITER //

CREATE OR REPLACE TRIGGER item_piece_before_insert
BEFORE INSERT ON item_piece
    FOR EACH ROW
    BEGIN
        DECLARE slot_occupied TINYINT;
        DECLARE item_slot ENUM('Head', 'Chest', 'Hands', 'Legs', 'Feet', 'MainHand', 'OffHand');
        # Ensure that exactly one of character_id and inventory_id is not null
        IF
            NEW.character_id IS NOT NULL AND NEW.inventory_id IS NOT NULL OR
            NEW.character_id IS NULL AND NEW.inventory_id IS NULL THEN
                SIGNAL SQLSTATE '45000'
                    SET MESSAGE_TEXT = 'Item piece must belong to a character or inventory';
        END IF;

        SELECT slot INTO item_slot
        FROM item WHERE id = NEW.item_id;

        IF item_slot IS NULL THEN
            SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'item_slot is NULL';
        END IF;

        SELECT EXISTS(
            SELECT * FROM item_piece
            JOIN item i
            WHERE character_id = NEW.character_id AND
            i.slot = item_slot
        ) INTO slot_occupied;

        IF slot_occupied THEN
            SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Gear slot is already occupied';
        END IF;

        IF item_slot = 'MainHand' OR item_slot = 'OffHand' THEN
            IF NEW.damage_lower IS NULL OR
               NEW.damage_upper IS NULL OR
               NEW.damage_lower > NEW.damage_upper OR
               NEW.armor_lower IS NOT NULL OR
               NEW.armor_upper IS NOT NULL THEN
                    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'A weapon must only have damage values';
            END IF;
        ELSE
            IF NEW.armor_lower IS NULL OR
               NEW.armor_upper IS NULL OR
               NEW.armor_lower > NEW.armor_upper OR
               NEW.damage_lower IS NOT NULL OR
               NEW.damage_upper IS NOT NULL THEN
                    SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Armor must only have armor values';
            END IF;
        END IF;

    END;

DELIMITER ;

DROP TRIGGER IF EXISTS item_piece_before_insert;