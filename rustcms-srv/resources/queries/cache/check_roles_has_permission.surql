(SELECT VALUE true 
FROM cache_role_permissions 
WHERE 
    name in $roles.map(|$item| $item.lowercase()) AND 
    permissions.any($permission.lowercase())
).any(true);