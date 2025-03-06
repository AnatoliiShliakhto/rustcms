BEGIN TRANSACTION;

REMOVE TABLE IF EXISTS cache_role_permissions;
DEFINE TABLE cache_role_permissions SCHEMAFULL;
DEFINE FIELD name ON TABLE cache_role_permissions TYPE string;
DEFINE FIELD permissions ON TABLE cache_role_permissions TYPE array<string>;

COMMIT TRANSACTION;