SELECT
    id.id() as id,
    (->rel_account_roles->core_roles.id).map(|$item| $item.id()) as roles
FROM ONLY core_accounts
WHERE
    login = $login AND
    crypto::argon2::compare(password, $password)
LIMIT 1;