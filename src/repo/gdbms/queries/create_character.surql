BEGIN TRANSACTION;

LET $acc = (SELECT VALUE id FROM type::thing("account", $account_id));
IF !$acc {
    THROW "Account not found";
};

LET $cls = (SELECT VALUE id FROM type::thing("class", $class_id));
IF !$cls {
    THROW "Invalid character class";
};

LET $char_id = INSERT INTO character (name, experience) VALUES ($name, $experience) RETURN VALUE id;

RELATE $char_id->is_a->$cls;
RELATE $acc->has->$char_id;
RETURN $char_id;

COMMIT TRANSACTION;