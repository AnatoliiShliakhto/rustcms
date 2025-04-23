BEGIN TRANSACTION;

REMOVE TABLE IF EXISTS core_accounts;
REMOVE TABLE IF EXISTS core_refresh_tokens;
REMOVE TABLE IF EXISTS core_roles;
REMOVE TABLE IF EXISTS core_permissions;
REMOVE TABLE IF EXISTS rel_account_refresh_tokens;
REMOVE TABLE IF EXISTS rel_account_roles;
REMOVE TABLE IF EXISTS rel_role_permissions;

REMOVE FUNCTION IF EXISTS fn::metadata;
REMOVE FUNCTION IF EXISTS fn::metadata::get;
REMOVE FUNCTION IF EXISTS fn::metadata::rec;
REMOVE FUNCTION IF EXISTS fn::metadata::create;
REMOVE FUNCTION IF EXISTS fn::metadata::update;

REMOVE PARAM IF EXISTS $role_administrator_rec;
REMOVE PARAM IF EXISTS $role_registered_rec;
REMOVE PARAM IF EXISTS $db_migrations;

RETURN true;

COMMIT TRANSACTION;