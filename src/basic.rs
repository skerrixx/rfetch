use std::process::Command;
use sysinfo::System;
use gfxinfo;
use whoami;
use starship_battery::Manager;
use starship_battery::units::ratio::percent;

fn os_id_or_name() -> String {
    if std::fs::exists("/bedrock/strata/bedrock/etc/os-release").unwrap() {
        let content = match std::fs::read_to_string("/bedrock/strata/bedrock/etc/os-release") {
            Ok(c) => c,
            Err(_) => return String::from(""),
        };

        fn extract_value(content: &str, key: &str) -> Option<String> {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix(&format!("{}=", key)) {
                    let v = rest.trim();
                    return Some(
                        v.trim_start_matches('"')
                            .trim_end_matches('"')
                            .to_string(),
                    );
                }
            }
            None
        }

        extract_value(&content, "ID")
            .or_else(|| extract_value(&content, "NAME"))
            .unwrap_or_default()
    } else {
        let content = match std::fs::read_to_string("/etc/os-release") {
            Ok(c) => c,
            Err(_) => return String::from(""),
        };

        fn extract_value(content: &str, key: &str) -> Option<String> {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix(&format!("{}=", key)) {
                    let v = rest.trim();
                    return Some(
                        v.trim_start_matches('"')
                            .trim_end_matches('"')
                            .to_string(),
                    );
                }
            }
            None
        }

        extract_value(&content, "ID")
            .or_else(|| extract_value(&content, "NAME"))
            .unwrap_or_default()
    }
}

fn normalize(name: &str) -> String {
    name.trim().to_lowercase()
}

fn format(os: &str) -> &'static str {
    match os {
        "arch" => "󰣇 arch",


        "debian" => " debian",
        "ubuntu" => "󰕈 ubuntu",
        "mint" => "󰣭 mint",
        "kali" => "  kali (larp final boss)",
        "raspbian" => " raspbian",

        "fedora" => " fedora",
        "rhel" | "redhat" | "red hat enterprise linux" => "  rhel",
        "centos" => " centos",
        "rocky" | "rocky linux" => "  rocky",
        "almalinux" | "alma linux" => "  alma",

        "opensuse-tumbleweed" | "opensuse tumbleweed" => " tumbleweed",
        "opensuse-leap" | "opensuse leap" => " leap",
        "sles" => " sles",
        "cachyos" => " cachy",

        "gentoo" => " gentoo",
        "void" | "voidlinux" => " void",
        "nixos" => " nix",
        "pop" | "popos" | "pop_os" | "pop os" => "  pop",
        "elementary" | "elementary os" => " elementary",
        "mageia" => " mageia",
        "openmandriva" => " openmandriva",
        "lfs" => "󰐱  linux from scratch",
        "bedrock" => "󰆦  bedrock",
        "rfetch" => "  rfetch",
        _ => "  linux (unknown)",
    }
}

pub fn display_name_for(name: &str) -> &'static str {
    let v = normalize(name);
    format(&v)
}

pub fn get_ascii_art(name: &str) -> &'static str {
    let v = normalize(name);
    match v.as_str() {
        "arch" | "archlinux" => r#"
                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `
"#,
        "ubuntu" => r#"
                             ....
              .',:clooo:  .:looooo:.
           .;looooooooc  .oooooooooo'
        .;looooool:,''.  :ooooooooooc
       ;looool;.         'oooooooooo,
      ;clool'             .cooooooc.  ,,
         ...                ......  .:oo,
  .;clol:,.                        .loooo'
 :ooooooooo,                        'ooool
'ooooooooooo.                        loooo.
'ooooooooool                         coooo.
 ,loooooooc.                        .loooo.
   .,;;;'.                          ;ooooc
       ...                         ,ooool.
    .cooooc.              ..',,'.  .cooo.
      ;ooooo:.           ;oooooooc.  :l.
       .coooooc,..      coooooooooo.
         .:ooooooolc:. .ooooooooooo'
           .':loooooo;  ,oooooooooc
               ..';::c'  .;loooo:'
"#,
        "debian" => r#"
        _,met$$$$$gg.
     ,g$$$$$$$$$$$$$$$P.
   ,g$$P``       ``'Y$$.
  ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,`'   .    $$$
 $$P      d     ,    $$P
 $$:      $   -    ,d$$'
 $$;      Y._   _,d'
 Y$$.    `.``Y$$$$P`'
 `$$b      "-.__
  `Y$$b
   `Y$$.
     `$$b.
       `Y$$b.
         `"Y$b._
             ``''-
"#,
        "fedora" => r#"
             .',-------'.
         .';:-------------;,.
      .;----------------------;.
    .:--------------------------:.
  .;--------------.:dddl:.--------;.
 .:-------------;OWMKOOXMWd;-------:.
.:-------------;KMMc;--;xMMc--------:.
,--------------;MMM.----;WW:---------,
:--------------;MMM.;----------------:
--------;oxOOO-;MMM000k.-------------:
------;0MMKxdd-;MMMkddc`-------------;
-----;XMO';----;MMM------------------'
-----;MMo;-----;MMW-----------------;
-----;0MNc.---.xMMd;---------------;
-------dNMWXXXWM0P---------------:,
--------RMMWMWMWP--------------:,.
------------------------------'
:----------------------------
 ':---------------------

"#,
        "rhel" | "redhat" | "red hat enterprise linux" => r#"  ____
           .MMM..:MMMMMMM
          MMMMMMMMMMMMMMMMMM
          MMMMMMMMMMMMMMMMMMMM.
         MMMMMMMMMMMMMMMMMMMMMM
        ,MMMMMMMMMMMMMMMMMMMMMM:
        MMMMMMMMMMMMMMMMMMMMMMMM
  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM
 MMMMMM    `MMMMMMMMMMMMMMMMMMMM.
MMMMMMMM      MMMMMMMMMMMMMMMMMM .
MMMMMMMMM.       `MMMMMMMMMMMMM' MM.
MMMMMMMMMMM.                     MMMM
`MMMMMMMMMMMMM.                 ,MMMMM.
 `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM.
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:
         MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
            `MMMMMMMMMMMMMMMMMMMMMMMM:
                ``MMMMMMMMMMMMMMMMM'
"#,
        "bedrock" => r#"
  \\\\\\\\
   \\    \\
    \\    \\
     \\    \\\\\\\\\\\\\\\     
      \\                 \\
       \\                 \\
        \\     ________    \\
         \\                //
          \\              //
           \\//////////////    
"#,
		"gentoo" => r#"
         -/oyddmdhs+:.
     -odNMMMMMMMMNNmhy+-`
   -yNMMMMMMMMMMMNNNmmdhy+-
 `omMMMMMMMMMMMMNmdmmmmddhhy/`
 omMMMMMMMMMMMN-'''yyohmdddhhhdo`
.ydMMMMMMMMMMd..  ../smdddhhhhdm+`
 oyhdmNMMMMMMMNb-.-ddmddddhhhhyhNd.
  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh
    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy
       /mMMMMMMNNNmmmdddhhhhhmMNhs:
    `oNMMMMMMMNNNmmmddddhhdmMNhs+`
  `sNMMMMMMMMNNNmmmdddddmNMmhs/.
 /NMMMMMMMMNNNNmmmdddmNMNdso:`
+MMMMMMMNNNNNmmmmdmNMNdso/-
yMMNNNNNNNmmmmmNNMmhs+/-`
/hMMNNNNNNNNMNdhs++/-`
`/ohdmmddhys+++/:.`
  `-//////:--.  
"#,
	"mint" => r#"
             ...-:::::-...
          .-MMMMMMMMMMMMMMM-.
      .-MMMM`..-:::::::-..`MMMM-.
    .:MMMM.:MMMMMMMMMMMMMMM:.MMMM:.
   -MMM-M---MMMMMMMMMMMMMMMMMMM.MMM-
  :MMM:MM`  :MMMM:....::-...-MMMM:MMM:
 :MMM:MMM`  :MM:`  ``    ``  `:MMM:MMM:
.MMM.MMMM`  :MM.  -MM.  .MM-  `MMMM.MMM.
:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:
:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM:MMM:
:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:
.MMM.MMMM`  :MM:--:MM:--:MM:  `MMMM.MMM.
 :MMM:MMM-  `-MMMMMMMMMMMM-`  -MMM-MMM:
  :MMM:MMM:`                `:MMM:MMM:
   .MMM.MMMM:--------------:MMMM.MMM.
     '-MMMM.-MMMMMMMMMMMMMMM-.MMMM-'
       '.-MMMM``--:::::--``MMMM-.'
            '-MMMMMMMMMMMMM-'
               ``-:::::-``  
"#,
	"lfs" => r#"
            :@@@@@@@:
            @@@@@@@@@-
    .:%.    @@@@@@@@@+.       @%
   *@@@%+:  :@@@@@@@%=: .=%@@@@@@=
  :@@@@@@##@@@@@@@@@%*+%@%+@@@@@@@+
  @@#####+@@@@@@@%:------=@@@@@@@@@-
 *@%#-----.@@@@@-----------@@@@@@@@#.
 %@--.@=:--+@@@@----%@:=---*@#*+=-+#:
 @@.-@@*=:--%%**-,.%@@%**----=-
 @@--@@@@+.-...:=.-%@@@@%-----
 %@%-#*#:.o.....o...-%@+-,--@+    -:
 +@@*.....................-@@@@@@@@+
  @%:....................._:@@@@@@@=.
  .=:...............__*-=`.=@@@@@@#=.
   :+:....:==*__*-=`:..==-:#@@@@@%+:
     .--=-:  +..::.....-:    =%@*=:
              :........-
                .:...--.  
"#,
	"kali" => r#"
..........
        ..,;:ccc,.
      ......''';lxO.
.''''..........,:ld;
       .';;;:::;,,.x,
  ..'''.            0Xxoc:,.  ...
..                ,ONkc;,;cokOdc',.
                 OMo           ':ddo.
                dMc               :OO;
                0M.                 .:o.
                ;Wd
                 ;XO,
                   ,d0Odlc;,..
                       ..',;:cdOOd::,.
                                .:d;.':;.
                                   'd,  .'
                                     ;l   ..
                                      .o
                                        c
                                        .'
                                        .  
"#,
	"cachyos" => r#"
     ...........
    /-++======/
   /++-++====/   ()
  /==++-+/
 /====++/     /''\
/======/      \,,/
\,,,,,,\            ,_,
 \,,....\          /   \
  \...,..\_________`-_-'
   \+=============/
    \+++=========/
"#,
	"rfetch" => r#"
     ....
 .-oOXNNX0d,.
.xMMMWKKNMMM0.
xMMMl    ;WMMK
0MMM'    .XMMW  .'lxkxc
xMMMNdc:oXMMM:  kMMMMMMMk
xMMMMMMMMMWW'  dMMM   `''
dMMM;oMMMN'    XMMML.,.
dMMM, 'XMMWl.  NMMMMMMM:
dMMM,  kMMMk   KMMM`
oMMM'   :WMMb  dMMM,
`TMP'   `TMP'  `TMT'
	"#,
        _ => r#"  ___
         _nnnn_        
        dGGGGMMb       
       @p~qp~~qMb  .-. 
       M|o||o)MM|   .' 
       @,----.JM|   o  
      JS^\__/  qKL     
     dZP        qKRb   
    dZP          qKKb  
   fZP            SMMb 
   HZM            MMMM 
   FqM            MMMM 
 __| ".        |\d/\qML
 |    `.       | `' \Zq
_)      \.___.,|     .'
\____   )MMMMMP|   .'  
     `-'       `--' 
"#,
    }
}

pub fn get_logo_color(name: &str) -> (u8, u8, u8) {
    let v = normalize(name);
    match v.as_str() {
        "arch" | "archlinux" => (96, 197, 255),
        "ubuntu" => (233, 84, 32),  
        "debian" => (215, 10, 83),  
        "fedora" => (60, 150, 230),  
        "rhel" | "redhat" | "red hat enterprise linux" => (238, 0, 0),
        "centos" => (35, 61, 99), 
        "rocky" | "rocky linux" => (16, 185, 129),
        "almalinux" | "alma linux" => (0, 42, 140), 
        "linuxmint" | "mint" => (135, 250, 62), 
        "kali" => (155, 194, 248), 
        "raspbian" => (180, 0, 72),  
        "opensuse-tumbleweed" | "opensuse tumbleweed" => (115, 186, 37),
        "opensuse-leap" | "opensuse leap" => (115, 186, 37),
        "sles" => (0, 153, 204),
        "pop" | "popos" | "pop_os" | "pop os" => (72, 169, 197),
        "elementary" | "elementary os" => (100, 186, 171),
        "void" | "voidlinux" => (71, 128, 97),
        "nixos" => (82, 119, 195),
        "mageia" => (47, 95, 143),
        "openmandriva" => (0, 153, 204),
        "gentoo" => (217, 200, 255),
        "lfs" => (255, 234, 174), 
        "bedrock" => (160, 160, 160), 
        "cachyos" => (3, 219, 209),
        "rfetch" => (100, 230, 255), 
        _ => (255, 255, 255), 
    }
}

pub fn known_distros() -> Vec<&'static str> {
    vec![
        "arch", "debian", "ubuntu", "linuxmint", "kali", "raspbian",
        "fedora", "rhel", "centos", "rocky", "almalinux",
        "opensuse-tumbleweed", "opensuse-leap", "sles",
        "gentoo", "void", "nixos", "pop", "elementary", "mageia",
        "openmandriva", "lfs", "bedrock", "rfetch", "cachyos"
    ]
}

pub fn cpu() -> String {
    let sys = System::new_all();
    let mut ret = String::from("");
    if let Some(first_cpu) = sys.cpus().first() {
        ret = first_cpu.brand().to_string();
    }
    ret
}

pub fn raw_os_id_or_name() -> String {
    os_id_or_name()
}


pub fn os() -> String {
    let v = os_id_or_name();
    format(&v).to_string()
}

pub fn ramuse() -> String {
    let sys = System::new_all();
    let mut used = sys.used_memory() as f32;
    used = used / 1024.0 / 1024.0 / 1024.0;
    used = (used * 10.0).ceil() / 10.0;
    used.to_string()
}

pub fn ramtotal() -> String {
    let sys = System::new_all();
    let mut total = sys.total_memory() as f32;
    total = total / 1024.0 / 1024.0 / 1024.0;
    total = (total * 10.0).ceil() / 10.0;
    total.to_string()
}

pub fn rampercent() -> String {
    let used_gb: f32 = ramuse().parse().unwrap_or(0.0);
    let total_gb: f32 = ramtotal().parse().unwrap_or(0.0);

    if total_gb <= 0.0 {
        return "0".to_string();
    }

    ((used_gb / total_gb) * 100.0).round().to_string()
}

pub fn disktot() -> u64 {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disk = &disks.list()[0];
    let gb = 1024 * 1024 * 1024;
    disk.total_space() / gb
}

pub fn diskuse() -> u64 {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disk = &disks.list()[0];
    let gb = 1024 * 1024 * 1024;
    (disk.total_space() - disk.available_space()) / gb
}

pub fn kernel() -> String {
    let output = Command::new("uname").arg("-sr").output().expect("");
    let ret = String::from_utf8_lossy(&output.stdout).into_owned();
    ret.trim().to_string()
}

pub fn gpu() -> String {
    if let Ok(gpu) = gfxinfo::active_gpu() {
        return gpu.model().to_string();
    }

    "none found, maybe integrated".to_string()
}

pub fn hostusr() -> String {
    format!(
        "{} ( {} )",
        whoami::username().to_string(),
        System::host_name().unwrap()
    )
}

pub fn uptime() -> String {
    let output = Command::new("uptime").arg("-p").output().expect("");
    let ret = String::from_utf8_lossy(&output.stdout).into_owned();
    ret.trim().to_string().replace("up ", "")
}

pub fn get_battery_charge() -> usize {
    let manager = match Manager::new() {
        Ok(m) => m,
        Err(_) => return 500,
    };

    if let Ok(mut battery_list) = manager.batteries() {
        if let Some(Ok(battery)) = battery_list.next() {
            let raw_percent: f32 = battery.state_of_charge().get::<percent>();
            return raw_percent as usize;
        }
    }

    500
}

