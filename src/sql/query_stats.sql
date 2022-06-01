SELECT *
FROM(
        SELECT Count(*)
        FROM users
    ),
    (
        SELECT Sum(tries)
        FROM problems
    ),
    (
        SELECT COUNT(*)
        FROM problems
        WHERE state >= 2
    ),
    (
        SELECT Sum(end_time - start_time)
        FROM problems
        WHERE end_time IS NOT NULL
    );