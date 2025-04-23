UPDATE ONLY core_refresh_tokens 
MERGE {
    expiration_at: $expiration_at
} 
WHERE 
    refresh_token == <uuid> $token_id
    AND expiration_at > time::unix()
RETURN VALUE <string> refresh_token;
