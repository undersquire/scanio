use std::io::{BufRead, BufReader, Read};

// TODO: Finish implementing input formatting
pub fn _read<T: std::str::FromStr>(stream: &mut dyn std::io::Read, pattern: &str) -> Result<T, ()> {
    static EOF: &[u8] = b"\t\r\n ";

    let pattern: Vec<&str> = pattern.split("{}").collect();

    // read input until one of the EOF bytes is found
    let mut input = String::new();
    loop {
        let mut buf = [0; 1];

        stream.read_exact(&mut buf).unwrap();

        if EOF.contains(&buf[0]) {
            break;
        }

        input.push(buf[0] as char);
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
    ) => {
        $crate::_read::<$t>($s, "{}")
    };

    (
        $s:expr,
        $p:literal,
        $t:ty
    ) => {
        $crate::_read::<$t>($s, $p)
    };

    (
        $s:expr,
        $($t:ty),*
    ) => {{
        #[inline]
        fn val() -> Result<($($t),*), ()> {
            Ok(
                (
                    $(match $crate::_read::<$t>($s, "{}") {
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
            Ok(
                (
                    $(match $crate::_read::<$t>($s, $p) {
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
    ) => {
        $i = $crate::_read($s, "{}").unwrap_or(Default::default());
    };

    (
        $s:expr,
        $p:literal,
        $i:ident
    ) => {
        $i = $crate::_read($s, $p).unwrap_or(Default::default());
    };

    (
        $s:expr,
        $($i:ident),*
    ) => {{
        $(
            $i = $crate::_read($s, "{}").unwrap_or(Default::default());
        )*
    }};

    (
        $s:expr,
        $p:literal,
        $($i:ident),*
    ) => {{
        $(
            $i = $crate::_read($s, $p).unwrap_or(Default::default());
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
