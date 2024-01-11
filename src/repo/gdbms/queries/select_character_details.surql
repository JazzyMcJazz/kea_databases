LET $char = type::thing("character", $id);

SELECT 
    *,
    string::split(type::string(array::first(<-has<-account.id)), ":")[1] as account_id,
    array::first(->is_a.out.*) as class,
    (SELECT
        *,
        array::first(->is_instance_of->item.name) AS name,
        array::first(->is_instance_of->item.rarity) AS rarity,
        array::first(->is_instance_of->item.slot) AS slot,
        array::first(->is_instance_of->item.weapon_skill) AS weapon_skill
    FROM $char->equipped->item_piece) AS equipped_gear,
    (SELECT
        *,
        array::first(->is_instance_of->item.name) AS name,
        array::first(->is_instance_of->item.rarity) AS rarity,
        array::first(->is_instance_of->item.slot) AS slot,
        array::first(->is_instance_of->item.weapon_skill) AS weapon_skill
    FROM $char->unequipped->item_piece) AS inventory
FROM $char;