fn _next_fmt(pattern: &mut String) -> String {
    let split: Vec<&str> = pattern.split("{}").collect();
    let mut result = String::new();

    if !split.is_empty() {
        result += split[0];
    }

    result += "{}";

    // remove from main pattern buffer
    if !split.is_empty() {
        let mut remove = split[0].to_string();
        remove += "{}";

        if split.len() > 1 {
            result += split[1];

            if let Some(c) = split[1].chars().next() {
                remove += c.to_string().as_str();
            }
        }

        *pattern = pattern.replace(&remove, "");
    }

    result
}

pub fn _read<T: std::str::FromStr>(
    stream: &mut dyn std::io::Read,
    pattern: &mut String,
) -> Result<T, ()> {
    let format = _next_fmt(pattern);

    let mut pattern = format.bytes();
    let mut input = String::new();

    'parse: loop {
        match pattern.next() {
            Some(b'{') => match pattern.next() {
                Some(b'}') => {
                    static EOF: &[u8] = b"\t\r\n ";
                    let next = pattern.next();

                    loop {
                        let mut buf = [0_u8];

                        let buf = match stream.read_exact(&mut buf) {
                            Ok(_) => Some(buf[0]),
                            Err(_) => None,
                        };

                        match buf {
                            Some(x) => {
                                if let Some(next) = next {
                                    if x == next {
                                        break 'parse;
                                    }
                                } else {
                                    if EOF.contains(&x) {
                                        break 'parse;
                                    }
                                }

                                if !EOF.contains(&x) {
                                    input.push(x as char);
                                }
                            }
                            None => return Err(()),
                        }
                    }
                }
                Some(_) => return Err(()),
                None => return Err(()),
            },
            Some(c) => {
                let mut buf = [0_u8];

                let buf = match stream.read_exact(&mut buf) {
                    Ok(_) => Some(buf[0]),
                    Err(_) => None,
                };

                match buf {
                    Some(b) => {
                        if c != b {
                            return Err(());
                        }
                    }
                    None => return Err(()),
                }
            }
            None => return Err(()),
        }
    }

    match input.parse::<T>() {
        Ok(value) => Ok(value),
        Err(_) => Err(()),
    }
}

#[macro_export]
macro_rules! try_read {
    (
        $s:expr,
        $t:ty
    ) => {{
        $crate::_read::<$t>($s, &mut "{}".to_string())
    }};

    (
        $s:expr,
        $p:literal,
        $t:ty
    ) => {
        $crate::_read::<$t>($s, &mut $p.to_string())
    };

    (
        $s:expr,
        $($t:ty),*
    ) => {{
        #[inline]
        fn val() -> Result<($($t),*), ()> {
            let mut fmt = "{}".to_string();
            Ok(
                (
                    $(match $crate::_read::<$t>($s, &mut fmt) {
                        Ok(x) => x,
                        Err(_) => return Err(()),
                    },)*
                )
            )
        }

        val()
    }};

    (
        $s:expr,
        $p:literal,
        $($t:ty),*
    ) => {{
        #[inline]
        fn val() -> Result<($($t),*), ()> {
            let mut fmt = $p.to_string();
            Ok(
                (
                    $(match $crate::_read::<$t>($s, &mut fmt) {
                        Ok(x) => x,
                        Err(_) => return Err(()),
                    },)*
                )
            )
        }

        val()
    }};
}

#[macro_export]
macro_rules! read {
    (
        $s:expr,
        $i:ident
    ) => {{
        $i = $crate::_read($s, &mut "{}".to_string()).unwrap_or(Default::default());
    }};

    (
        $s:expr,
        $p:literal,
        $i:ident
    ) => {{
        $i = $crate::_read($s, &mut $p.to_string()).unwrap_or(Default::default());
    }};

    (
        $s:expr,
        $($i:ident),*
    ) => {{
        let mut fmt = "{}".to_string();
        $(
            $i = $crate::_read($s, &mut fmt).unwrap_or(Default::default());
        )*
    }};

    (
        $s:expr,
        $p:literal,
        $($i:ident),*
    ) => {{
        let mut fmt = $p.to_string();
        $(
            $i = $crate::_read($s, &mut fmt).unwrap_or(Default::default());
        )*
    }};
}

#[macro_export]
macro_rules! try_scan {
    (
        $t:ty
    ) => {
        try_read!(&mut std::io::stdin(), $t)
    };

    (
        $p:literal,
        $t:ty
    ) => {
        try_read!(&mut std::io::stdin(), $p, $t)
    };

    (
        $($t:ty),*
    ) => {
        try_read!(&mut std::io::stdin(), $($t),*)
    };

    (
        $p:literal,
        $($t:ty),*
    ) => {
        try_read!(&mut std::io::stdin(), $p, $($t),*)
    };
}

#[macro_export]
macro_rules! scan {
    (
        $i:ident
    ) => {
        read!(&mut std::io::stdin(), $i)
    };

    (
        $p:literal,
        $i:ident
    ) => {
        read!(&mut std::io::stdin(), $p, $i)
    };

    (
        $($i:ident),*
    ) => {
        read!(&mut std::io::stdin(), $($i),*)
    };

    (
        $p:literal,
        $($i:ident),*
    ) => {
        read!(&mut std::io::stdin(), $p, $($i),*)
    };
}
