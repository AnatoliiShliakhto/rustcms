BEGIN TRANSACTION;

LET $acc_password = IF $password != none THEN { crypto::argon2::generate($password) } ELSE { none } END;

LET $account_rec = CREATE ONLY core_accounts CONTENT {
    login: $login,
    password: $acc_password
} RETURN VALUE id;

fn::metadata::update(id, $creator_id);

RELATE $account_rec->rel_account_roles->$role_registered_rec;

RETURN true;

COMMIT TRANSACTION;