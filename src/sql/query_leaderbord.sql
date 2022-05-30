SELECT problem_id,
    name,
    avatar_url,
    end_time,
    login
FROM problems
    JOIN users u on problems.user_id = u.id
WHERE state >= 2
    AND end_time IS NOT NULL
ORDER BY end_time DESC;