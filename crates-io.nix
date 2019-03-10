{ lib, buildRustCrate, buildRustCrateHelpers }:
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
in
rec {

# bitflags-1.0.3

  crates.bitflags."1.0.3" = deps: { features?(features_.bitflags."1.0.3" deps {}) }: buildRustCrate {
    crateName = "bitflags";
    version = "1.0.3";
    authors = [ "The Rust Project Developers" ];
    sha256 = "162p4w4h1ad76awq6b5yivmls3d50m9cl27d8g588lsps6g8s5rw";
    features = mkFeatures (features."bitflags"."1.0.3" or {});
  };
  features_.bitflags."1.0.3" = deps: f: updateFeatures f (rec {
    bitflags."1.0.3".default = (f.bitflags."1.0.3".default or true);
  }) [];


# end
# cfg-if-0.1.4

  crates.cfg_if."0.1.4" = deps: { features?(features_.cfg_if."0.1.4" deps {}) }: buildRustCrate {
    crateName = "cfg-if";
    version = "0.1.4";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "0n5baxk53dvqjymzwynq55wb805b24390qx1n16zi8fjzq90j7k4";
  };
  features_.cfg_if."0.1.4" = deps: f: updateFeatures f (rec {
    cfg_if."0.1.4".default = (f.cfg_if."0.1.4".default or true);
  }) [];


# end
# chrono-0.4.4

  crates.chrono."0.4.4" = deps: { features?(features_.chrono."0.4.4" deps {}) }: buildRustCrate {
    crateName = "chrono";
    version = "0.4.4";
    authors = [ "Kang Seonghoon <public+rust@mearie.org>" "Brandon W Maister <quodlibetor@gmail.com>" ];
    sha256 = "169h5rlrb9df3yvnzihjw39cjipvz90qgf9211pivms2s9xnpqpr";
    dependencies = mapFeatures features ([
      (crates."num_integer"."${deps."chrono"."0.4.4"."num_integer"}" deps)
      (crates."num_traits"."${deps."chrono"."0.4.4"."num_traits"}" deps)
    ]
      ++ (if features.chrono."0.4.4".serde or false then [ (crates.serde."${deps."chrono"."0.4.4".serde}" deps) ] else [])
      ++ (if features.chrono."0.4.4".time or false then [ (crates.time."${deps."chrono"."0.4.4".time}" deps) ] else []));
    features = mkFeatures (features."chrono"."0.4.4" or {});
  };
  features_.chrono."0.4.4" = deps: f: updateFeatures f (rec {
    chrono = fold recursiveUpdate {} [
      { "0.4.4".clock =
        (f.chrono."0.4.4".clock or false) ||
        (f.chrono."0.4.4".default or false) ||
        (chrono."0.4.4"."default" or false); }
      { "0.4.4".default = (f.chrono."0.4.4".default or true); }
      { "0.4.4".time =
        (f.chrono."0.4.4".time or false) ||
        (f.chrono."0.4.4".clock or false) ||
        (chrono."0.4.4"."clock" or false); }
    ];
    num_integer."${deps.chrono."0.4.4".num_integer}".default = (f.num_integer."${deps.chrono."0.4.4".num_integer}".default or false);
    num_traits."${deps.chrono."0.4.4".num_traits}".default = (f.num_traits."${deps.chrono."0.4.4".num_traits}".default or false);
    serde."${deps.chrono."0.4.4".serde}".default = true;
    time."${deps.chrono."0.4.4".time}".default = true;
  }) [
    (features_.num_integer."${deps."chrono"."0.4.4"."num_integer"}" deps)
    (features_.num_traits."${deps."chrono"."0.4.4"."num_traits"}" deps)
    (features_.serde."${deps."chrono"."0.4.4"."serde"}" deps)
    (features_.time."${deps."chrono"."0.4.4"."time"}" deps)
  ];


# end
# dimensioned-0.7.0

  crates.dimensioned."0.7.0" = deps: { features?(features_.dimensioned."0.7.0" deps {}) }: buildRustCrate {
    crateName = "dimensioned";
    version = "0.7.0";
    authors = [ "Paho Lurie-Gregg <paho@paholg.com>" ];
    sha256 = "1qr5v55i8drj78411q1plmq6b5s9mv6r67c58v8mh2wb0n7lffjh";
    build = "src/build/mod.rs";
    dependencies = mapFeatures features ([
      (crates."generic_array"."${deps."dimensioned"."0.7.0"."generic_array"}" deps)
      (crates."num_traits"."${deps."dimensioned"."0.7.0"."num_traits"}" deps)
      (crates."typenum"."${deps."dimensioned"."0.7.0"."typenum"}" deps)
    ]
      ++ (if features.dimensioned."0.7.0".serde or false then [ (crates.serde."${deps."dimensioned"."0.7.0".serde}" deps) ] else []));
    features = mkFeatures (features."dimensioned"."0.7.0" or {});
  };
  features_.dimensioned."0.7.0" = deps: f: updateFeatures f (rec {
    dimensioned = fold recursiveUpdate {} [
      { "0.7.0".approx =
        (f.dimensioned."0.7.0".approx or false) ||
        (f.dimensioned."0.7.0".test or false) ||
        (dimensioned."0.7.0"."test" or false); }
      { "0.7.0".ci =
        (f.dimensioned."0.7.0".ci or false) ||
        (f.dimensioned."0.7.0".test or false) ||
        (dimensioned."0.7.0"."test" or false); }
      { "0.7.0".clapme =
        (f.dimensioned."0.7.0".clapme or false) ||
        (f.dimensioned."0.7.0".test or false) ||
        (dimensioned."0.7.0"."test" or false); }
      { "0.7.0".default = (f.dimensioned."0.7.0".default or true); }
      { "0.7.0".quickcheck =
        (f.dimensioned."0.7.0".quickcheck or false) ||
        (f.dimensioned."0.7.0".test or false) ||
        (dimensioned."0.7.0"."test" or false); }
      { "0.7.0".serde =
        (f.dimensioned."0.7.0".serde or false) ||
        (f.dimensioned."0.7.0".test or false) ||
        (dimensioned."0.7.0"."test" or false); }
      { "0.7.0".serde_test =
        (f.dimensioned."0.7.0".serde_test or false) ||
        (f.dimensioned."0.7.0".test or false) ||
        (dimensioned."0.7.0"."test" or false); }
      { "0.7.0".std =
        (f.dimensioned."0.7.0".std or false) ||
        (f.dimensioned."0.7.0".default or false) ||
        (dimensioned."0.7.0"."default" or false); }
    ];
    generic_array."${deps.dimensioned."0.7.0".generic_array}".default = true;
    num_traits."${deps.dimensioned."0.7.0".num_traits}".default = (f.num_traits."${deps.dimensioned."0.7.0".num_traits}".default or false);
    serde."${deps.dimensioned."0.7.0".serde}".default = true;
    typenum."${deps.dimensioned."0.7.0".typenum}".default = true;
  }) [
    (features_.generic_array."${deps."dimensioned"."0.7.0"."generic_array"}" deps)
    (features_.num_traits."${deps."dimensioned"."0.7.0"."num_traits"}" deps)
    (features_.serde."${deps."dimensioned"."0.7.0"."serde"}" deps)
    (features_.typenum."${deps."dimensioned"."0.7.0"."typenum"}" deps)
  ];


# end
# dtoa-0.4.3

  crates.dtoa."0.4.3" = deps: { features?(features_.dtoa."0.4.3" deps {}) }: buildRustCrate {
    crateName = "dtoa";
    version = "0.4.3";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1xysdxdm24sk5ysim7lps4r2qaxfnj0sbakhmps4d42yssx30cw8";
  };
  features_.dtoa."0.4.3" = deps: f: updateFeatures f (rec {
    dtoa."0.4.3".default = (f.dtoa."0.4.3".default or true);
  }) [];


# end
# fuchsia-zircon-0.3.3

  crates.fuchsia_zircon."0.3.3" = deps: { features?(features_.fuchsia_zircon."0.3.3" deps {}) }: buildRustCrate {
    crateName = "fuchsia-zircon";
    version = "0.3.3";
    authors = [ "Raph Levien <raph@google.com>" ];
    sha256 = "0jrf4shb1699r4la8z358vri8318w4mdi6qzfqy30p2ymjlca4gk";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."fuchsia_zircon"."0.3.3"."bitflags"}" deps)
      (crates."fuchsia_zircon_sys"."${deps."fuchsia_zircon"."0.3.3"."fuchsia_zircon_sys"}" deps)
    ]);
  };
  features_.fuchsia_zircon."0.3.3" = deps: f: updateFeatures f (rec {
    bitflags."${deps.fuchsia_zircon."0.3.3".bitflags}".default = true;
    fuchsia_zircon."0.3.3".default = (f.fuchsia_zircon."0.3.3".default or true);
    fuchsia_zircon_sys."${deps.fuchsia_zircon."0.3.3".fuchsia_zircon_sys}".default = true;
  }) [
    (features_.bitflags."${deps."fuchsia_zircon"."0.3.3"."bitflags"}" deps)
    (features_.fuchsia_zircon_sys."${deps."fuchsia_zircon"."0.3.3"."fuchsia_zircon_sys"}" deps)
  ];


# end
# fuchsia-zircon-sys-0.3.3

  crates.fuchsia_zircon_sys."0.3.3" = deps: { features?(features_.fuchsia_zircon_sys."0.3.3" deps {}) }: buildRustCrate {
    crateName = "fuchsia-zircon-sys";
    version = "0.3.3";
    authors = [ "Raph Levien <raph@google.com>" ];
    sha256 = "08jp1zxrm9jbrr6l26bjal4dbm8bxfy57ickdgibsqxr1n9j3hf5";
  };
  features_.fuchsia_zircon_sys."0.3.3" = deps: f: updateFeatures f (rec {
    fuchsia_zircon_sys."0.3.3".default = (f.fuchsia_zircon_sys."0.3.3".default or true);
  }) [];


# end
# generic-array-0.11.1

  crates.generic_array."0.11.1" = deps: { features?(features_.generic_array."0.11.1" deps {}) }: buildRustCrate {
    crateName = "generic-array";
    version = "0.11.1";
    authors = [ "Bartłomiej Kamiński <fizyk20@gmail.com>" "Aaron Trent <novacrazy@gmail.com>" ];
    sha256 = "1fiyqqmfxll9j67sw2j1c64mr7njbw7cl4j9xsckpah3brhhhj1x";
    libName = "generic_array";
    dependencies = mapFeatures features ([
      (crates."typenum"."${deps."generic_array"."0.11.1"."typenum"}" deps)
    ]);
  };
  features_.generic_array."0.11.1" = deps: f: updateFeatures f (rec {
    generic_array."0.11.1".default = (f.generic_array."0.11.1".default or true);
    typenum."${deps.generic_array."0.11.1".typenum}".default = true;
  }) [
    (features_.typenum."${deps."generic_array"."0.11.1"."typenum"}" deps)
  ];


# end
# itoa-0.4.2

  crates.itoa."0.4.2" = deps: { features?(features_.itoa."0.4.2" deps {}) }: buildRustCrate {
    crateName = "itoa";
    version = "0.4.2";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "193a744yd74rmk13hl8xvd9p2hqhdkyf8xkvi1mxm5s10bby0h8v";
    features = mkFeatures (features."itoa"."0.4.2" or {});
  };
  features_.itoa."0.4.2" = deps: f: updateFeatures f (rec {
    itoa = fold recursiveUpdate {} [
      { "0.4.2".default = (f.itoa."0.4.2".default or true); }
      { "0.4.2".std =
        (f.itoa."0.4.2".std or false) ||
        (f.itoa."0.4.2".default or false) ||
        (itoa."0.4.2"."default" or false); }
    ];
  }) [];


# end
# libc-0.2.42

  crates.libc."0.2.42" = deps: { features?(features_.libc."0.2.42" deps {}) }: buildRustCrate {
    crateName = "libc";
    version = "0.2.42";
    authors = [ "The Rust Project Developers" ];
    sha256 = "064v49hz1zpl081w8c4vwikrkhaxp06y4i9l7x7wx6bjpwp19pjx";
    features = mkFeatures (features."libc"."0.2.42" or {});
  };
  features_.libc."0.2.42" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "0.2.42".default = (f.libc."0.2.42".default or true); }
      { "0.2.42".use_std =
        (f.libc."0.2.42".use_std or false) ||
        (f.libc."0.2.42".default or false) ||
        (libc."0.2.42"."default" or false); }
    ];
  }) [];


# end
# linked-hash-map-0.5.1

  crates.linked_hash_map."0.5.1" = deps: { features?(features_.linked_hash_map."0.5.1" deps {}) }: buildRustCrate {
    crateName = "linked-hash-map";
    version = "0.5.1";
    authors = [ "Stepan Koltsov <stepan.koltsov@gmail.com>" "Andrew Paseltiner <apaseltiner@gmail.com>" ];
    sha256 = "1f29c7j53z7w5v0g115yii9dmmbsahr93ak375g48vi75v3p4030";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."linked_hash_map"."0.5.1" or {});
  };
  features_.linked_hash_map."0.5.1" = deps: f: updateFeatures f (rec {
    linked_hash_map = fold recursiveUpdate {} [
      { "0.5.1".default = (f.linked_hash_map."0.5.1".default or true); }
      { "0.5.1".heapsize =
        (f.linked_hash_map."0.5.1".heapsize or false) ||
        (f.linked_hash_map."0.5.1".heapsize_impl or false) ||
        (linked_hash_map."0.5.1"."heapsize_impl" or false); }
      { "0.5.1".serde =
        (f.linked_hash_map."0.5.1".serde or false) ||
        (f.linked_hash_map."0.5.1".serde_impl or false) ||
        (linked_hash_map."0.5.1"."serde_impl" or false); }
      { "0.5.1".serde_test =
        (f.linked_hash_map."0.5.1".serde_test or false) ||
        (f.linked_hash_map."0.5.1".serde_impl or false) ||
        (linked_hash_map."0.5.1"."serde_impl" or false); }
    ];
  }) [];


# end
# num-integer-0.1.39

  crates.num_integer."0.1.39" = deps: { features?(features_.num_integer."0.1.39" deps {}) }: buildRustCrate {
    crateName = "num-integer";
    version = "0.1.39";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1f42ls46cghs13qfzgbd7syib2zc6m7hlmv1qlar6c9mdxapvvbg";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."num_traits"."${deps."num_integer"."0.1.39"."num_traits"}" deps)
    ]);
    features = mkFeatures (features."num_integer"."0.1.39" or {});
  };
  features_.num_integer."0.1.39" = deps: f: updateFeatures f (rec {
    num_integer = fold recursiveUpdate {} [
      { "0.1.39".default = (f.num_integer."0.1.39".default or true); }
      { "0.1.39".std =
        (f.num_integer."0.1.39".std or false) ||
        (f.num_integer."0.1.39".default or false) ||
        (num_integer."0.1.39"."default" or false); }
    ];
    num_traits = fold recursiveUpdate {} [
      { "${deps.num_integer."0.1.39".num_traits}"."i128" =
        (f.num_traits."${deps.num_integer."0.1.39".num_traits}"."i128" or false) ||
        (num_integer."0.1.39"."i128" or false) ||
        (f."num_integer"."0.1.39"."i128" or false); }
      { "${deps.num_integer."0.1.39".num_traits}"."std" =
        (f.num_traits."${deps.num_integer."0.1.39".num_traits}"."std" or false) ||
        (num_integer."0.1.39"."std" or false) ||
        (f."num_integer"."0.1.39"."std" or false); }
      { "${deps.num_integer."0.1.39".num_traits}".default = (f.num_traits."${deps.num_integer."0.1.39".num_traits}".default or false); }
    ];
  }) [
    (features_.num_traits."${deps."num_integer"."0.1.39"."num_traits"}" deps)
  ];


# end
# num-traits-0.2.5

  crates.num_traits."0.2.5" = deps: { features?(features_.num_traits."0.2.5" deps {}) }: buildRustCrate {
    crateName = "num-traits";
    version = "0.2.5";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0ql203ca6lzppksy4fsfnpz3kq96vwlwvyn3ahvnd9g6k9f5ncj0";
    build = "build.rs";
    features = mkFeatures (features."num_traits"."0.2.5" or {});
  };
  features_.num_traits."0.2.5" = deps: f: updateFeatures f (rec {
    num_traits = fold recursiveUpdate {} [
      { "0.2.5".default = (f.num_traits."0.2.5".default or true); }
      { "0.2.5".std =
        (f.num_traits."0.2.5".std or false) ||
        (f.num_traits."0.2.5".default or false) ||
        (num_traits."0.2.5"."default" or false); }
    ];
  }) [];


# end
# proc-macro2-0.4.17

  crates.proc_macro2."0.4.17" = deps: { features?(features_.proc_macro2."0.4.17" deps {}) }: buildRustCrate {
    crateName = "proc-macro2";
    version = "0.4.17";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "0py0zn1xws3csi3p7nmiqaqdqxkns0vlyiaq6f4rhhvx4ml202w1";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."unicode_xid"."${deps."proc_macro2"."0.4.17"."unicode_xid"}" deps)
    ]);
    features = mkFeatures (features."proc_macro2"."0.4.17" or {});
  };
  features_.proc_macro2."0.4.17" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "0.4.17".default = (f.proc_macro2."0.4.17".default or true); }
      { "0.4.17".proc-macro =
        (f.proc_macro2."0.4.17".proc-macro or false) ||
        (f.proc_macro2."0.4.17".default or false) ||
        (proc_macro2."0.4.17"."default" or false) ||
        (f.proc_macro2."0.4.17".nightly or false) ||
        (proc_macro2."0.4.17"."nightly" or false); }
    ];
    unicode_xid."${deps.proc_macro2."0.4.17".unicode_xid}".default = true;
  }) [
    (features_.unicode_xid."${deps."proc_macro2"."0.4.17"."unicode_xid"}" deps)
  ];


# end
# quote-0.6.8

  crates.quote."0.6.8" = deps: { features?(features_.quote."0.6.8" deps {}) }: buildRustCrate {
    crateName = "quote";
    version = "0.6.8";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0dq6j23w6pmc4l6v490arixdwypy0b82z76nrzaingqhqri4p3mh";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."quote"."0.6.8"."proc_macro2"}" deps)
    ]);
    features = mkFeatures (features."quote"."0.6.8" or {});
  };
  features_.quote."0.6.8" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.quote."0.6.8".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.quote."0.6.8".proc_macro2}"."proc-macro" or false) ||
        (quote."0.6.8"."proc-macro" or false) ||
        (f."quote"."0.6.8"."proc-macro" or false); }
      { "${deps.quote."0.6.8".proc_macro2}".default = (f.proc_macro2."${deps.quote."0.6.8".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "0.6.8".default = (f.quote."0.6.8".default or true); }
      { "0.6.8".proc-macro =
        (f.quote."0.6.8".proc-macro or false) ||
        (f.quote."0.6.8".default or false) ||
        (quote."0.6.8"."default" or false); }
    ];
  }) [
    (features_.proc_macro2."${deps."quote"."0.6.8"."proc_macro2"}" deps)
  ];


# end
# rand-0.4.2

  crates.rand."0.4.2" = deps: { features?(features_.rand."0.4.2" deps {}) }: buildRustCrate {
    crateName = "rand";
    version = "0.4.2";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0h8pkg23wb67i8904sm76iyr1jlmhklb85vbpz9c9191a24xzkfm";
    dependencies = (if kernel == "fuchsia" then mapFeatures features ([
      (crates."fuchsia_zircon"."${deps."rand"."0.4.2"."fuchsia_zircon"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
    ]
      ++ (if features.rand."0.4.2".libc or false then [ (crates.libc."${deps."rand"."0.4.2".libc}" deps) ] else [])) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."rand"."0.4.2"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."rand"."0.4.2" or {});
  };
  features_.rand."0.4.2" = deps: f: updateFeatures f (rec {
    fuchsia_zircon."${deps.rand."0.4.2".fuchsia_zircon}".default = true;
    libc."${deps.rand."0.4.2".libc}".default = true;
    rand = fold recursiveUpdate {} [
      { "0.4.2".default = (f.rand."0.4.2".default or true); }
      { "0.4.2".i128_support =
        (f.rand."0.4.2".i128_support or false) ||
        (f.rand."0.4.2".nightly or false) ||
        (rand."0.4.2"."nightly" or false); }
      { "0.4.2".libc =
        (f.rand."0.4.2".libc or false) ||
        (f.rand."0.4.2".std or false) ||
        (rand."0.4.2"."std" or false); }
      { "0.4.2".std =
        (f.rand."0.4.2".std or false) ||
        (f.rand."0.4.2".default or false) ||
        (rand."0.4.2"."default" or false); }
    ];
    winapi = fold recursiveUpdate {} [
      { "${deps.rand."0.4.2".winapi}"."minwindef" = true; }
      { "${deps.rand."0.4.2".winapi}"."ntsecapi" = true; }
      { "${deps.rand."0.4.2".winapi}"."profileapi" = true; }
      { "${deps.rand."0.4.2".winapi}"."winnt" = true; }
      { "${deps.rand."0.4.2".winapi}".default = true; }
    ];
  }) [
    (features_.fuchsia_zircon."${deps."rand"."0.4.2"."fuchsia_zircon"}" deps)
    (features_.libc."${deps."rand"."0.4.2"."libc"}" deps)
    (features_.winapi."${deps."rand"."0.4.2"."winapi"}" deps)
  ];


# end
# redox_syscall-0.1.40

  crates.redox_syscall."0.1.40" = deps: { features?(features_.redox_syscall."0.1.40" deps {}) }: buildRustCrate {
    crateName = "redox_syscall";
    version = "0.1.40";
    authors = [ "Jeremy Soller <jackpot51@gmail.com>" ];
    sha256 = "132rnhrq49l3z7gjrwj2zfadgw6q0355s6a7id7x7c0d7sk72611";
    libName = "syscall";
  };
  features_.redox_syscall."0.1.40" = deps: f: updateFeatures f (rec {
    redox_syscall."0.1.40".default = (f.redox_syscall."0.1.40".default or true);
  }) [];


# end
# serde-1.0.70

  crates.serde."1.0.70" = deps: { features?(features_.serde."1.0.70" deps {}) }: buildRustCrate {
    crateName = "serde";
    version = "1.0.70";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1z1gyjf5rrs1k3j1civfzqjqs790651bwf8m31bw2pagclhnazs4";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."serde"."1.0.70" or {});
  };
  features_.serde."1.0.70" = deps: f: updateFeatures f (rec {
    serde = fold recursiveUpdate {} [
      { "1.0.70".default = (f.serde."1.0.70".default or true); }
      { "1.0.70".serde_derive =
        (f.serde."1.0.70".serde_derive or false) ||
        (f.serde."1.0.70".derive or false) ||
        (serde."1.0.70"."derive" or false); }
      { "1.0.70".std =
        (f.serde."1.0.70".std or false) ||
        (f.serde."1.0.70".default or false) ||
        (serde."1.0.70"."default" or false); }
      { "1.0.70".unstable =
        (f.serde."1.0.70".unstable or false) ||
        (f.serde."1.0.70".alloc or false) ||
        (serde."1.0.70"."alloc" or false); }
    ];
  }) [];


# end
# serde_derive-1.0.76

  crates.serde_derive."1.0.76" = deps: { features?(features_.serde_derive."1.0.76" deps {}) }: buildRustCrate {
    crateName = "serde_derive";
    version = "1.0.76";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "153mayfyrp101kpf1ccxljm923j4hr4r0ngxx53ikpxjvb5gz469";
    procMacro = true;
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."serde_derive"."1.0.76"."proc_macro2"}" deps)
      (crates."quote"."${deps."serde_derive"."1.0.76"."quote"}" deps)
      (crates."syn"."${deps."serde_derive"."1.0.76"."syn"}" deps)
    ]);
    features = mkFeatures (features."serde_derive"."1.0.76" or {});
  };
  features_.serde_derive."1.0.76" = deps: f: updateFeatures f (rec {
    proc_macro2."${deps.serde_derive."1.0.76".proc_macro2}".default = true;
    quote."${deps.serde_derive."1.0.76".quote}".default = true;
    serde_derive."1.0.76".default = (f.serde_derive."1.0.76".default or true);
    syn = fold recursiveUpdate {} [
      { "${deps.serde_derive."1.0.76".syn}"."visit" = true; }
      { "${deps.serde_derive."1.0.76".syn}".default = true; }
    ];
  }) [
    (features_.proc_macro2."${deps."serde_derive"."1.0.76"."proc_macro2"}" deps)
    (features_.quote."${deps."serde_derive"."1.0.76"."quote"}" deps)
    (features_.syn."${deps."serde_derive"."1.0.76"."syn"}" deps)
  ];


# end
# serde_json-1.0.24

  crates.serde_json."1.0.24" = deps: { features?(features_.serde_json."1.0.24" deps {}) }: buildRustCrate {
    crateName = "serde_json";
    version = "1.0.24";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1wvvc3y0202my2p00ah8ygl1794nspar9pf39fz1525jd6m6k8a1";
    dependencies = mapFeatures features ([
      (crates."dtoa"."${deps."serde_json"."1.0.24"."dtoa"}" deps)
      (crates."itoa"."${deps."serde_json"."1.0.24"."itoa"}" deps)
      (crates."serde"."${deps."serde_json"."1.0.24"."serde"}" deps)
    ]);
    features = mkFeatures (features."serde_json"."1.0.24" or {});
  };
  features_.serde_json."1.0.24" = deps: f: updateFeatures f (rec {
    dtoa."${deps.serde_json."1.0.24".dtoa}".default = true;
    itoa."${deps.serde_json."1.0.24".itoa}".default = true;
    serde."${deps.serde_json."1.0.24".serde}".default = true;
    serde_json = fold recursiveUpdate {} [
      { "1.0.24".default = (f.serde_json."1.0.24".default or true); }
      { "1.0.24".indexmap =
        (f.serde_json."1.0.24".indexmap or false) ||
        (f.serde_json."1.0.24".preserve_order or false) ||
        (serde_json."1.0.24"."preserve_order" or false); }
    ];
  }) [
    (features_.dtoa."${deps."serde_json"."1.0.24"."dtoa"}" deps)
    (features_.itoa."${deps."serde_json"."1.0.24"."itoa"}" deps)
    (features_.serde."${deps."serde_json"."1.0.24"."serde"}" deps)
  ];


# end
# syn-0.14.9

  crates.syn."0.14.9" = deps: { features?(features_.syn."0.14.9" deps {}) }: buildRustCrate {
    crateName = "syn";
    version = "0.14.9";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1ia0qbrnqz40s8886b2jpcjiqfbziigd96lqjfin06xk6i28vr7b";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."syn"."0.14.9"."proc_macro2"}" deps)
      (crates."unicode_xid"."${deps."syn"."0.14.9"."unicode_xid"}" deps)
    ]
      ++ (if features.syn."0.14.9".quote or false then [ (crates.quote."${deps."syn"."0.14.9".quote}" deps) ] else []));
    features = mkFeatures (features."syn"."0.14.9" or {});
  };
  features_.syn."0.14.9" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.syn."0.14.9".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.syn."0.14.9".proc_macro2}"."proc-macro" or false) ||
        (syn."0.14.9"."proc-macro" or false) ||
        (f."syn"."0.14.9"."proc-macro" or false); }
      { "${deps.syn."0.14.9".proc_macro2}".default = (f.proc_macro2."${deps.syn."0.14.9".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "${deps.syn."0.14.9".quote}"."proc-macro" =
        (f.quote."${deps.syn."0.14.9".quote}"."proc-macro" or false) ||
        (syn."0.14.9"."proc-macro" or false) ||
        (f."syn"."0.14.9"."proc-macro" or false); }
      { "${deps.syn."0.14.9".quote}".default = (f.quote."${deps.syn."0.14.9".quote}".default or false); }
    ];
    syn = fold recursiveUpdate {} [
      { "0.14.9".clone-impls =
        (f.syn."0.14.9".clone-impls or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".default = (f.syn."0.14.9".default or true); }
      { "0.14.9".derive =
        (f.syn."0.14.9".derive or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".parsing =
        (f.syn."0.14.9".parsing or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".printing =
        (f.syn."0.14.9".printing or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".proc-macro =
        (f.syn."0.14.9".proc-macro or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".quote =
        (f.syn."0.14.9".quote or false) ||
        (f.syn."0.14.9".printing or false) ||
        (syn."0.14.9"."printing" or false); }
    ];
    unicode_xid."${deps.syn."0.14.9".unicode_xid}".default = true;
  }) [
    (features_.proc_macro2."${deps."syn"."0.14.9"."proc_macro2"}" deps)
    (features_.quote."${deps."syn"."0.14.9"."quote"}" deps)
    (features_.unicode_xid."${deps."syn"."0.14.9"."unicode_xid"}" deps)
  ];


# end
# time-0.1.40

  crates.time."0.1.40" = deps: { features?(features_.time."0.1.40" deps {}) }: buildRustCrate {
    crateName = "time";
    version = "0.1.40";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0wgnbjamljz6bqxsd5axc4p2mmhkqfrryj4gf2yswjaxiw5dd01m";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."time"."0.1.40"."libc"}" deps)
    ])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."time"."0.1.40"."redox_syscall"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."time"."0.1.40"."winapi"}" deps)
    ]) else []);
  };
  features_.time."0.1.40" = deps: f: updateFeatures f (rec {
    libc."${deps.time."0.1.40".libc}".default = true;
    redox_syscall."${deps.time."0.1.40".redox_syscall}".default = true;
    time."0.1.40".default = (f.time."0.1.40".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.time."0.1.40".winapi}"."minwinbase" = true; }
      { "${deps.time."0.1.40".winapi}"."minwindef" = true; }
      { "${deps.time."0.1.40".winapi}"."ntdef" = true; }
      { "${deps.time."0.1.40".winapi}"."profileapi" = true; }
      { "${deps.time."0.1.40".winapi}"."std" = true; }
      { "${deps.time."0.1.40".winapi}"."sysinfoapi" = true; }
      { "${deps.time."0.1.40".winapi}"."timezoneapi" = true; }
      { "${deps.time."0.1.40".winapi}".default = true; }
    ];
  }) [
    (features_.libc."${deps."time"."0.1.40"."libc"}" deps)
    (features_.redox_syscall."${deps."time"."0.1.40"."redox_syscall"}" deps)
    (features_.winapi."${deps."time"."0.1.40"."winapi"}" deps)
  ];


# end
# typenum-1.10.0

  crates.typenum."1.10.0" = deps: { features?(features_.typenum."1.10.0" deps {}) }: buildRustCrate {
    crateName = "typenum";
    version = "1.10.0";
    authors = [ "Paho Lurie-Gregg <paho@paholg.com>" "Andre Bogus <bogusandre@gmail.com>" ];
    sha256 = "1v2cgg0mlzkg5prs7swysckgk2ay6bpda8m83c2sn3z77dcsx3bc";
    build = "build/main.rs";
    features = mkFeatures (features."typenum"."1.10.0" or {});
  };
  features_.typenum."1.10.0" = deps: f: updateFeatures f (rec {
    typenum."1.10.0".default = (f.typenum."1.10.0".default or true);
  }) [];


# end
# unicode-xid-0.1.0

  crates.unicode_xid."0.1.0" = deps: { features?(features_.unicode_xid."0.1.0" deps {}) }: buildRustCrate {
    crateName = "unicode-xid";
    version = "0.1.0";
    authors = [ "erick.tryzelaar <erick.tryzelaar@gmail.com>" "kwantam <kwantam@gmail.com>" ];
    sha256 = "05wdmwlfzxhq3nhsxn6wx4q8dhxzzfb9szsz6wiw092m1rjj01zj";
    features = mkFeatures (features."unicode_xid"."0.1.0" or {});
  };
  features_.unicode_xid."0.1.0" = deps: f: updateFeatures f (rec {
    unicode_xid."0.1.0".default = (f.unicode_xid."0.1.0".default or true);
  }) [];


# end
# uuid-0.6.5

  crates.uuid."0.6.5" = deps: { features?(features_.uuid."0.6.5" deps {}) }: buildRustCrate {
    crateName = "uuid";
    version = "0.6.5";
    authors = [ "Ashley Mannix<ashleymannix@live.com.au>" "Christopher Armstrong" "Dylan DPC<dylan.dpc@gmail.com>" "Hunar Roop Kahlon<hunar.roop@gmail.com>" ];
    sha256 = "1jy15m4yxxwma0jsy070garhbgfprky23i77rawjkk75vqhnnhlf";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."uuid"."0.6.5"."cfg_if"}" deps)
    ]
      ++ (if features.uuid."0.6.5".rand or false then [ (crates.rand."${deps."uuid"."0.6.5".rand}" deps) ] else [])
      ++ (if features.uuid."0.6.5".serde or false then [ (crates.serde."${deps."uuid"."0.6.5".serde}" deps) ] else []));
    features = mkFeatures (features."uuid"."0.6.5" or {});
  };
  features_.uuid."0.6.5" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.uuid."0.6.5".cfg_if}".default = true;
    rand."${deps.uuid."0.6.5".rand}".default = true;
    serde."${deps.uuid."0.6.5".serde}".default = (f.serde."${deps.uuid."0.6.5".serde}".default or false);
    uuid = fold recursiveUpdate {} [
      { "0.6.5".byteorder =
        (f.uuid."0.6.5".byteorder or false) ||
        (f.uuid."0.6.5".u128 or false) ||
        (uuid."0.6.5"."u128" or false); }
      { "0.6.5".default = (f.uuid."0.6.5".default or true); }
      { "0.6.5".md5 =
        (f.uuid."0.6.5".md5 or false) ||
        (f.uuid."0.6.5".v3 or false) ||
        (uuid."0.6.5"."v3" or false); }
      { "0.6.5".nightly =
        (f.uuid."0.6.5".nightly or false) ||
        (f.uuid."0.6.5".const_fn or false) ||
        (uuid."0.6.5"."const_fn" or false); }
      { "0.6.5".rand =
        (f.uuid."0.6.5".rand or false) ||
        (f.uuid."0.6.5".v3 or false) ||
        (uuid."0.6.5"."v3" or false) ||
        (f.uuid."0.6.5".v4 or false) ||
        (uuid."0.6.5"."v4" or false) ||
        (f.uuid."0.6.5".v5 or false) ||
        (uuid."0.6.5"."v5" or false); }
      { "0.6.5".sha1 =
        (f.uuid."0.6.5".sha1 or false) ||
        (f.uuid."0.6.5".v5 or false) ||
        (uuid."0.6.5"."v5" or false); }
      { "0.6.5".std =
        (f.uuid."0.6.5".std or false) ||
        (f.uuid."0.6.5".default or false) ||
        (uuid."0.6.5"."default" or false) ||
        (f.uuid."0.6.5".use_std or false) ||
        (uuid."0.6.5"."use_std" or false); }
    ];
  }) [
    (features_.cfg_if."${deps."uuid"."0.6.5"."cfg_if"}" deps)
    (features_.rand."${deps."uuid"."0.6.5"."rand"}" deps)
    (features_.serde."${deps."uuid"."0.6.5"."serde"}" deps)
  ];


# end
# winapi-0.3.5

  crates.winapi."0.3.5" = deps: { features?(features_.winapi."0.3.5" deps {}) }: buildRustCrate {
    crateName = "winapi";
    version = "0.3.5";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0cfdsxa5yf832r5i2z7dhdvnryyvhfp3nb32gpcaq502zgjdm3w6";
    build = "build.rs";
    dependencies = (if kernel == "i686-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_i686_pc_windows_gnu"."${deps."winapi"."0.3.5"."winapi_i686_pc_windows_gnu"}" deps)
    ]) else [])
      ++ (if kernel == "x86_64-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_x86_64_pc_windows_gnu"."${deps."winapi"."0.3.5"."winapi_x86_64_pc_windows_gnu"}" deps)
    ]) else []);
    features = mkFeatures (features."winapi"."0.3.5" or {});
  };
  features_.winapi."0.3.5" = deps: f: updateFeatures f (rec {
    winapi."0.3.5".default = (f.winapi."0.3.5".default or true);
    winapi_i686_pc_windows_gnu."${deps.winapi."0.3.5".winapi_i686_pc_windows_gnu}".default = true;
    winapi_x86_64_pc_windows_gnu."${deps.winapi."0.3.5".winapi_x86_64_pc_windows_gnu}".default = true;
  }) [
    (features_.winapi_i686_pc_windows_gnu."${deps."winapi"."0.3.5"."winapi_i686_pc_windows_gnu"}" deps)
    (features_.winapi_x86_64_pc_windows_gnu."${deps."winapi"."0.3.5"."winapi_x86_64_pc_windows_gnu"}" deps)
  ];


# end
# winapi-i686-pc-windows-gnu-0.4.0

  crates.winapi_i686_pc_windows_gnu."0.4.0" = deps: { features?(features_.winapi_i686_pc_windows_gnu."0.4.0" deps {}) }: buildRustCrate {
    crateName = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "05ihkij18r4gamjpxj4gra24514can762imjzlmak5wlzidplzrp";
    build = "build.rs";
  };
  features_.winapi_i686_pc_windows_gnu."0.4.0" = deps: f: updateFeatures f (rec {
    winapi_i686_pc_windows_gnu."0.4.0".default = (f.winapi_i686_pc_windows_gnu."0.4.0".default or true);
  }) [];


# end
# winapi-x86_64-pc-windows-gnu-0.4.0

  crates.winapi_x86_64_pc_windows_gnu."0.4.0" = deps: { features?(features_.winapi_x86_64_pc_windows_gnu."0.4.0" deps {}) }: buildRustCrate {
    crateName = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0n1ylmlsb8yg1v583i4xy0qmqg42275flvbc51hdqjjfjcl9vlbj";
    build = "build.rs";
  };
  features_.winapi_x86_64_pc_windows_gnu."0.4.0" = deps: f: updateFeatures f (rec {
    winapi_x86_64_pc_windows_gnu."0.4.0".default = (f.winapi_x86_64_pc_windows_gnu."0.4.0".default or true);
  }) [];


# end
# yaml-rust-0.4.0

  crates.yaml_rust."0.4.0" = deps: { features?(features_.yaml_rust."0.4.0" deps {}) }: buildRustCrate {
    crateName = "yaml-rust";
    version = "0.4.0";
    authors = [ "Yuheng Chen <yuhengchen@sensetime.com>" ];
    sha256 = "1mqv1jagn9hfym28ypp2dq6hw9kcyilzil9ydlpls0ivb8d9i3h8";
    dependencies = mapFeatures features ([
      (crates."linked_hash_map"."${deps."yaml_rust"."0.4.0"."linked_hash_map"}" deps)
    ]);
  };
  features_.yaml_rust."0.4.0" = deps: f: updateFeatures f (rec {
    linked_hash_map."${deps.yaml_rust."0.4.0".linked_hash_map}".default = true;
    yaml_rust."0.4.0".default = (f.yaml_rust."0.4.0".default or true);
  }) [
    (features_.linked_hash_map."${deps."yaml_rust"."0.4.0"."linked_hash_map"}" deps)
  ];


# end
}
