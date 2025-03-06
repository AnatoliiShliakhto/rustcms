BEGIN TRANSACTION;

LET $acc_password = IF $password != none THEN { crypto::argon2::generate($password) } ELSE { none } END;

LET $account_rec = CREATE ONLY core_accounts CONTENT {
    login: $login,
    password: $acc_password
} RETURN id;

LET $role_registered_rec = SELECT VALUE c_value FROM ONLY core_system WHERE c_key = 'role_registered' LIMIT 1;

RELATE $account_rec->rel_account_roles->$role_registered_rec;

COMMIT TRANSACTION;