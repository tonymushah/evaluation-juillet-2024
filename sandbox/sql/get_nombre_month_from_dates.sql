select
    (
        (
            (
                extract(
                    Month
                    from
                        date '2024/2/22'
                ) - 1
            ) - (
                extract(
                    Month
                    from
                        date '2024/02/1'
                ) - 1
            )
        ) % 11
    ) + (
        (
            extract(
                year
                from
                    date '2024/2/22'
            ) - extract(
                year
                from
                    date '2024/02/1'
            )
        ) * 11
    ) as result;