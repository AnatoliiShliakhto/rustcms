UPDATE ONLY core_refresh_tokens 
MERGE {
    expiration_at: <int> $expiration_at
} 
WHERE 
    refresh_token == <uuid> $token_id
RETURN VALUE <string> refresh_token;
