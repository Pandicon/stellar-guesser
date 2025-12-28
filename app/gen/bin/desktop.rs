fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
    stellar_guesser::main();
}
