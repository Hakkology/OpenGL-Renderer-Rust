fn main() {
    // Gelecekte native kütüphane derlemek için burası kullanılabilir.
    // Şimdilik sadece değişiklikleri izliyoruz.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
}
