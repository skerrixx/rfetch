use std::process::Command;
use std::path::Path;
use std::env;
use std::collections::HashSet;
use sysinfo::System;
use whoami;
use starship_battery::Manager;
use starship_battery::units::ratio::percent;

pub fn is_termux() -> bool {
    if env::var("TERMUX_VERSION").is_ok() {
        return true;
    }
    if let Ok(prefix) = env::var("PREFIX") {
        if prefix.contains("com.termux") {
            return true;
        }
    }
    std::fs::exists("/data/data/com.termux").unwrap_or(false)
}

fn os_id_or_name() -> String {
    if is_termux() {
        return String::from("android");
    }
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
    } else if std::fs::exists("/etc/os-release").unwrap() {
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
    else {
        let os = String::from_utf8(Command::new("uname").arg("s").output().expect("").stdout).expect("").to_string().to_lowercase();
        return os;
    }
}

fn normalize(name: &str) -> String {
    name.trim().to_lowercase()
}

fn format(os: &str) -> &'static str {
    match os {
    	"arch" => "󰣇 arch", // yes
		"artix" => "  artix", // yes
		"omarchy" => " omarchy", // yes
		"manjaro" => "  manjaro", // yes
		"endeavour" | "endeavouros" => "  endeavour", // yes
		"archlabs" => "  archlabs", // yes
		"archcraft" => "  archcraft", // yes
		"kyon" => " kyon linux", // yes
		"garuda" => "  garuda", // yes
        "debian" => " debian", // yes
        "ubuntu" => "󰕈 ubuntu", // yes
        "mint" => "󰣭 mint", // yes
        "kali" => "  kali (larp final boss)", // yes
        "raspbian" => " raspbian", // yes

        "fedora" => " fedora", // yes
        "rhel" | "redhat" | "red hat enterprise linux" => "  rhel", //yes
        "centos" => " centos", // yes
        "rocky" | "rocky linux" => "  rocky",
        "almalinux" | "alma linux" | "alma" => "  alma", // yes

        "opensuse-tumbleweed" | "opensuse tumbleweed" => " tumbleweed", // yes
        "opensuse-leap" | "opensuse leap" => " leap", // yes
        "sles" => " sles", // yes
        "cachyos" => " cachy", // yes

        "gentoo" => " gentoo", // yes
        "void" | "voidlinux" => " void", // yes
        "nixos" => " nix", // yes
        "pop" | "popos" | "pop_os" | "pop os" => "  pop", // yes
        "elementary" | "elementary os" => " elementary", // yes
        "mageia" => " mageia", // yes
        "openmandriva" => " openmandriva", // yes
        "lfs" => "󰐱  linux from scratch", // yes
        "bedrock" => "󰆦  bedrock", // yes
        "rfetch" => "  rfetch", // yes
        "mist" => "  mist", // yes
        "chimera" => "󱗽  chimera", // yes
        "alpine" => "  alpine", // yes
        "zerene" => "  zereneos",
        "netbsd" => "󰉀  netbsd",
        "android" => " android",
        _ => "  linux (unknown)", // yes
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
          /\
         /  \ 
        /    \ 
       _\     \ 
      /        \ 
     /          \ 
    /     __   \_\  
   /     /  \     \
  /__,--'    '--,__\
"#,
	"artix" | "artixlinux" => r#"
            '
           'A'
          'ooo'
         'ookxo'
         `ookxxo'
       '.   `ooko'
      'ooo`.   `oo'
     'ooxxxoo`.   `'
    'ookxxxkooo.`   .
   'ookxxkoo'`   .'oo'
  'ooxoo'`     .:ooxxo'
 'io'`             `'oo'
'`                     `'
"#,
	"manjaro" => r#"██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
████████            ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
"#,
	"omarchy" => r#"
████████████████████████████
██          ██            ██
██  ██████████      ████  ██
██  ██                ██  ██
██  ██                ██  ██
██  ██                ██  ██
██  ██                ██  ██
██████                ██  ██
██  ██                ██  ██
██  ██                ██  ██
██  ██                ██  ██
██  ████████████████████  ██
██           ██           ██
███████████████  ███████████
"#,
	"endeavouros" | "endeavour" => r#"                     ./o.
                   ./sssso-
                 `:osssssss+-
               `:+sssssssssso/.
             `-/ossssssssssssso/.
           `-/+sssssssssssssssso+:`
         `-:/+sssssssssssssssssso+/.
       `.://osssssssssssssssssssso++-
      .://+ssssssssssssssssssssssso++:
    .:///ossssssssssssssssssssssssso++:
  `:////ssssssssssssssssssssssssssso+++.
`-////+ssssssssssssssssssssssssssso++++-
 `..-+oosssssssssssssssssssssssso+++++/`
   ./++++++++++++++++++++++++++++++/:.
  `:::::::::::::::::::::::::------``
"#,
	"archlabs" => r#"                     'c'
                    'kKk,
                   .dKKKx.
                  .oKXKXKd.
                 .l0XXXXKKo.
                 c0KXXXXKX0l.
                :0XKKOxxOKX0l.
               :OXKOc. .c0XX0l.
              :OK0o. ...'dKKX0l.
             :OX0c  ;xOx''dKXX0l.
            :0KKo..o0XXKd'.lKXX0l.
           c0XKd..oKXXXXKd..oKKX0l.
         .c0XKk;.l0K0OO0XKd..oKXXKo.
        .l0XXXk:,dKx,.'l0XKo..kXXXKo.
       .o0XXXX0d,:x;   .oKKx'.dXKXXKd.
      .oKXXXXKK0c.;.    :00c'cOXXXXXKd.
     .dKXXXXXXXXk,.     cKx''xKXXXXXXKx'
    'xKXXXXK0kdl:.     .ok; .cdk0KKXXXKx'
   'xKK0koc,..         'c,     ..,cok0KKk,
  ,xko:'.             ..            .':okx;
 .,'.                                   .',.
"#,
	"archcraft" => r#"
                   ⢰⡆                     
                  ⢠⣿⣿⡄                  
                 ⢀⣾⣿⣿⣿⡀                 
                 ⣼⣿⣿⣿⣿⣷⡀                
                ⣼⣿⣿⣿⣿⣿⣿⣷                
               ⢼⣿⣿⣿⣿⣿⣿⣿⣿⣧               
              ⣰⣤⣈⠻⢿⣿⣿⣿⣿⣿⣿⣧              
             ⣰⣿⣿⣿⣿⣮⣿⣿⣿⣿⣿⣿⣿⣧             
            ⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧            
           ⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧           
          ⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧          
         ⣼⣿⣿⣿⣿⣿⡿⣿⣿⡟  ⠸⣿⣿⡿⣿⣿⣿⣿⣿⣷⡀        
        ⣼⣿⣿⣿⣿⣿⡏          ⠈⣿⣿⣿⣿⣿⣷⡀        
      ⢀⣼⣿⣿⣿⣿⣿⣿⡗   ⢀⣠⣤⣀   ⠸⣿⣿⣿⣿⣿⣿⣷⡀      
     ⢀⣾⣿⣿⣿⣿⣿⡏⠁   ⢠⣿⣿⣿⣿⡇    ⢙⣿⣿⣻⠿⣿⣷⡀     
    ⢀⣾⣿⣿⣿⣿⣿⣿⣷⣤⡀   ⠻⣿⣿⡿⠃   ⢀⣼⣿⣿⣿⣿⣦⣌⠙     
   ⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏           ⢿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀   
  ⢠⣿⣿⣿⣿⣿⣿⣿⡿⠟⠋⠁             ⠙⠻⣿⣿⣿⣿⣿⣿⣿⣿⡄  
 ⣠⣿⣿⣿⣿⠿⠛⠋⠁                    ⠉⠙⠻⢿⣿⣿⣿⣿⣆ 
⡰⠟⠛⠉⠁                              ⠉⠙⠛⠿⢆
"#,
	"kyon" => r#"
```                l                                 
              .dxxdolc.x.                         
             ;xxxxxxxxkK0oc;                      
            oxxxxxxxxk0K0x:'.oolc.                
          .dxxxxxxxxx0Kd    cxxxxxdolc'           
         'xxxodxxxxx0KO    ;kxxxxxxxxxxxdol:      
        cd'.   lxxxOKK'   .00kxxxxxxxxxxxxxxxdolc.
   cclldkko     cOOKKl    kK0xdldo0000000000Oxdd  
     ;KKKKKO.    :KKk    lOo;odxOOKKKKKKK0kxddc   
    :xxkO0KK0'    '0.     ';o0KKKKKKKK0Oxdddd'    
   oxxxxxxkO00;        .      .,lkK0Oxdddddd      
 .dxxxxxxxxxdxkc      :K0xc'       .,cddddo       
  odddddddddddd0d     oKKKKKKko;.      .:;        
       'ddddddkKKk.    cKKKKKKKKK0xc,:od.         
             d0KKK0.    'xO0KKKKKKKKOdd           
             oKKKKK0,  .'lddxkO00KKKKo            
            .KKo       .dddddddddxkOkK'           
                             oddddd  "#,
	"garuda" => r#"
             .:loooodddddd:.
           .:l:.         .cd:.
         .:l;.             .co:.
       .:l;.       ,o.       .:o:.
     .:l;.       'oddl::::::::;lll.
   .cc;.         ``````````````;ol.
 .:c,.                          :`
 .:l:.     ,,,,,,,,,,,,,,,,,.
   .clc. .::::cc::::::::clll;.
     .coc.             ,ll,.
       .cdl.        .,ll,
         .coooooollllc'
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
   ,g$$P""       """Y$$.".
  ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,"'   .      $$$
 $$P      d     ,     $$P
 $$:      $   -     ,d$$'
 $$;      Y._     _,d'
 Y$$.    `.`"Y$$$$P"'
 `$$b      "-.__
  `Y$$b
   `Y$$.
     `$$b.
       `Y$$b.
         `"Y$b._
             `""''
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
.ydMMMMMMMMMMd..  ../smdddhhhhiii`
 oyhdmNMMMMMMMNb-.-ddmddddhhhiiii;.
  :oyhhdNNMMMMMMMNNNmmdddhhhhiii;/;
    .:+sydNMMMMMNNNmmmdddhhiii;;;,,
       /mMMMMMMNNNmmmdddhiii;;;;;,,
    `oNMMMMMMMNNNmmmdddiii;;;;;,,`
  `sNMMMMMMMMNNNmmmdidiii;;;;;,.
 /NMMMMMMMMNNNNmmmddii;;;;;,,`
+MMMMMMMNNNNNmmmmiii;;;;;,,
yMMNNNNNNNmmmmmNiii;;;,,
/hMMNNNNNNNNMN;ii;,,,`
`/ohdmmddhys+ii;;.,
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
          .+=========================.
         :++===++==================-        :++-
        :*++====+++++=============-        .==:
       -*+++=====+***++==========:
      =*++++========------------:
     =*+++++=====-                     ...
   .+*+++++=-===:                    .=+++=:
  :++++=====-==:                     -*****+
 :++========-=.                      .=++**+.
.+==========-.                          .
:+++++++====-                                .--==-.
 :++==========.                             :+++++++:
  .-===========.                            =*****+*+
   .-===========:                           .+*****+:
     -=======++++:::::::::::::::::::::::::-:  .---:
      :======++++====+++******************=.
       :=====+++==========++++++++++++++*-
        .====++==============++++++++++*-
         .===+==================+++++++:
          .-=======================+++:
            ..........................
"#,
	"void" => r#"
            ⣀⣀⣠⣤⣴⣶⣿⣿⣿⣶⣶⣦⣤⣄⡀          
         ⢤⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿;.        
          ⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄      
     .     ⠹⣿⡿⠋⠉     ⠉⠙⠛⠿⣿⣿⣿⣿⣿⣿⣿⣆    
    :::,    ⠁            ⠈⠛⠿⣿⣿⣿⣿⣿⣧   
   ::::::.       _._        ⣿⣿⣿⣿⣿⣿⣧  
  .::::::`    +=======+     ⠸⣿⣿⣿⣿⣿⣿⣧ 
  ::::::;    ==========+     ⠘⣿⣿⣿⣿⣿⣿  
 :::::::    ============      ⣿⣿⣿⣿⣿⣿⡇
 :::::::    ============      ⣿⣿⣿⣿⣿⣿ 
  ::::::     ==========      ⣰⣿⣿⣿⣿⣿⣿ 
  -::::::      +====+       ⣰⣿⣿⣿⣿⣿⣿⠇ 
   +::::::,                 ⢻⣿⣿⣿⣿⣿⣿  
    +:::::::.           .    ⠙⢿⣿⣿⡟   
     -:::::::::..___..-::-.    ⠙⠋    
       +:::::::::::::::::::;.        
         ~:::::::::::::::::-`        
            -::::::::~               
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
	"centos" => r#"
                 ..
               .PLTJ.
              <><><><>
     KKSSV' 4KKK LJ KKKL.'VSSKK
     KKV' 4KKKKK LJ KKKKAL 'VKK
     V'   'VKKKK LJ KKKKV' ' 'V
     .4MA.  'VKK LJ KKV' '.4Mb.
   . KKKKKA.  'V LJ V' '.4KKKKK .
 .4D KKKKKKKA. ' LJ ''.4KKKKKKK FA.
<QDD ++++++++++++  ++++++++++++ GFD>
 'VD KKKKKKKK'   LJ ..'KKKKKKKK FV
   ' VKKKKK'  .4 LJ K. .'KKKKKV '
      'VK'  .4KK LJ KKA. .'KV'
     A.   .4KKKK LJ KKKKA. . .4
     KKA. 'KKKKK LJ KKKKK' .4KK
     KKSSA. VKKK LJ KKKV .4SSKK
              <><><><>
               'MKKM'
                 ''
"#,
	"pop" | "popos" => r#"             *CCC                
          CCCCCCCCCC*              
        CCCCC`'CCCCCCC              
       CCCCCC  `:CCCCC      _     
        CCCCCC   CCCCC.  ,dCCC*       
         CCCCCCb,CCCCC   CCCCCC        
          CCCCCCCCCCC    CCCCC'        
           CCCCCCCC      CCCC'        
            ,CCCC,       CCC'         
             *CCCC       P`            
               CCCC                    
                CCCC   767            
                                     
         .-ccccccccccccccccc-,      
         \CCCCCCCCCCCCCCCCCCC/
"#,
	"nix" | "nixos" => r#"
        __    ____    __
       /  \   \;;;\  /;;\
       \   \   \;;;\/;;;/
     ___\   \___\;;;;;;/
    /            \;;;;/   /\
   /______________\;;;\  /  \
        /;;;/      \;;;\/   /
 ______/;;;/        \;;/   /___
/;;;;;;;;;/          \/        \
\;;;;;;;;/\          /   ______/
    /;;;/  \        /   /
   /;;;/\   \______/___/_____
   \;;/  \   \;;;;;;;;;;;;;;/
    \/   /    \;;;;;;;;;;;;/
        /      \   \;;;\
       /   /\   \   \;;;\
       \__/  \___\   \;;/
"#,
	"opensuse-tumbleweed" | "opensuse-leap" | "sles" => r#"
           .;ldkO0000Okdl;.
       .;d00xl:^''''''^:ok00d;.
     .d00l'                'o00d.
   .d0Kd'  Okxol:;,.          
  .OKKKK0kOKKKKKKKKKKOxo:,      
 ,0KKKKKKKKKKKKKKKK0P^,,,^dx:   
.OKKKKKKKKKKKKKKKKk'.oOPPb.'0k. 
:KKKKKKKKKKKKKKKKK: kKx..dd lKd  
dKKKKKKKKKKKOx0KKKd ^0KKKO' kKKc 
dKKKKKKKKKKKK;.;oOKx,..^..;kKKK0.
:KKKKKKKKKKKK0o;...^cdxxOK0O/^^'  .0K:
 kKKKKKKKKKKKKKKK0x;,,......,;od  lKk
 '0KKKKKKKKKKKKKKKKKKKKK00KKOo^  c00'
  'kKKKOxddxkOO00000Okxoc;''   .dKk'
    l0Ko.                    .c00l'
     'l0Kk:.              .;xK0l'
        'lkK0xl:;,,,,;:ldO0kl'
            '^:ldxkkkkxdl:^'
"#,
	"mist" | "mistlinux" => r#"
               XMMMMMMc
            lMMMMMMMMMMMc
           ;MMMMd    MMMMo  ,-,
            WMMM:     MMW0dc:::::.
              XMMMW   KKc::.  .:::,
                      Mk:::.   .:::
                     MMMO  ..  .:::
                   MMMMd       :::.
 oMc           XMMMMM        .:::.
      WMMMMMMMMN           ,-::.
                   ,-:::::::''
              .,-::;,
"#,
	"openmandriva" => r#"
                 ``````
            `-:/+++++++//:-.`
         .:+++oooo+/:.``   ``
      `:+ooooooo+:.  `-:/++++++/:.`
     -+oooooooo:` `-++o+/::::://+o+/-
   `/ooooooooo-  -+oo/.`        `-/oo+.
  `+ooooooooo.  :os/`              .+so:
  +sssssssss/  :ss/                 `+ss-
 :ssssssssss`  sss`                  .sso
 ossssssssss  `yyo                    sys
`sssssssssss` `yys                   `yys
`sssssssssss:  +yy/                  +yy:
 oyyyyyyyyyys. `oyy/`              `+yy+
 :yyyyyyyyyyyo. `+yhs:.         `./shy/
  oyyyyyyyyyyys:` .oyhys+:----/+syhy+. `
  `syyyyyyyyyyyyo-` .:osyhhhhhyys+:``.:`
   `oyyyyyyyyyyyyys+-`` `.----.```./oo.
     /yhhhhhhhhhhhhhhyso+//://+osyhy/`
      `/yhhhhhhhhhhhhhhhhhhhhhhhhy/`
        `:oyhhhhhhhhhhhhhhhhhhyo:`
            .:+syhhhhhhhhys+:-`
                ``....``
"#,
	"mageia" => r#"        .°°.
         °°   .°°.
         .°°°. °°
         .   .
          °°° .°°°.
      .°°°.   '___'
     .'___'        .
   :dkxc;'.  ..,cxkd;
 .dkk. kkkkkkkkkk .kkd.
.dkk.  ';cloolc;.  .kkd
ckk.                .kk;
xO:                  cOd
xO:                  lOd
lOO.                .OO:
.k00.              .00x
 .k00;            ;00O.
  .lO0Kc;,,,,,,;c0KOc.
     ;d00KKKKKK00d;
        .,KKKK,.
"#,
	"alma" | "almalinux" => r#"         'c:.
        lkkkx, ..       ..   ,cc,
        okkkk:ckkx'  .lxkkx.okkkkd
        .:llcokkx'  :kkkxkko:xkkd,
      .xkkkkdood:  ;kx,  .lkxlll;
       xkkx.       xk'     xkkkkk:
       'xkx.       xd      .....,.
      .. :xkl'     :c      ..''..
    .dkx'  .:ldl:'. '  ':lollldkkxo;
  .''lkkko'                     ckkkx.
'xkkkd:kkd.       ..  ;'        :kkxo.
,xkkkd;kk'      ,d;    ld.   ':dkd::cc,
 .,,.;xkko'.';lxo.      dx,  :kkk'xkkkkc
     'dkkkkkxo:.        ;kx  .kkk:;xkkd.
       .....   .;dk:.   lkk.  :;,
             :kkkkkkkdoxkkx
              ,c,,;;;:xkkd.
                ;kkkkl...
                ;kkkkl
                 ,od;
"#,
	"rocky" => r#"      __wgliliiligw_,
       _williiiiiiliilililw,
     _%iiiiiilililiiiiiiiiiii_
   .Qliiiililiiiiiiililililiilm.
  _iiiiiliiiiiililiiiiiiiiiiliil,
 .lililiiilililiiiilililililiiiii,
_liiiiiiliiiiiiiliiiiiF{iiiiiilili,
jliililiiilililiiili@`  ~ililiiiiiL
iiiliiiiliiiiiiili>`      ~liililii
liliiiliiilililii`         -9liiiil
iiiiiliiliiiiii~             "4lili
4ililiiiiilil~|      -w,       )4lf
-liiiiililiF'       _liig,       )'
 )iiiliii@`       _QIililig,
  )iiii>`       .Qliliiiililw
   )<>~       .mliiiiiliiiiiil,
	  _gllilililiililii~
	 giliiiiiiiiiiiiT`
	-^~lilili@~~'
"#,
	"raspbian" => r#"`.::///+:/-.        --///+//-:`
 `+oooooooooooo:   `+oooooooooooo:
  /oooo++//ooooo:  ooooo+//+ooooo.
  `+ooooooo:-:oo-  +o+::/ooooooo:
    `:oooooooo+``    `.oooooooo+-
      `:++ooo/.        :+ooo+/.`
         ...`  `.----.`  ``..
      .::::-``:::::::::.`-:::-`
     -:::-`   .:::::::-`  `-:::-
    `::.  `.--.`  `` `.---.``.::`
        .::::::::`  -::::::::` `
  .::` .:::::::::- `::::::::::``::.
 -:::` ::::::::::.  ::::::::::.`:::-
 ::::  -::::::::.   `-::::::::  ::::
 -::-   .-:::-.``....``.-::-.   -::-
  .. ``       .::::::::.     `..`..
    -:::-`   -::::::::::`  .:::::`
    :::::::` -::::::::::` :::::::.
    .:::::::  -::::::::. ::::::::
     `-:::::`   ..--.`   ::::::.
       `...`  `...--..`  `...`
             .::::::::::
              `.-::::-`
    "#,
	"elementary" | "elementary os" | "testestest" => r#"         eeeeeeeeeeeeeeeee
      eeeeeeeeeeeeeeeeeeeeeee
    eeeee  eeeeeeeeeeee   eeeee
  eeee   eeeee       eee     eeee
 eeee   eeee          eee     eeee
eee    eee            eee       eee
eee   eee            eee        eee
ee    eee           eeee       eeee
ee    eee         eeeee      eeeeee
ee    eee       eeeee      eeeee ee
eee   eeee   eeeeee      eeeee  eee
eee    eeeeeeeeee     eeeeee    eee
 eeeeeeeeeeeeeeeeeeeeeeee    eeeee
  eeeeeeee eeeeeeeeeeee      eeee
    eeeee                 eeeee
      eeeeeee         eeeeeee
         eeeeeeeeeeeeeeeee
"#,
	"chimera" => r#"888888888888  888
888888888888  888
888888888888  888
88888888P"' _,888
888888P' ,jd88888
88888P  d88P'
8888b  j88'         xxxxxxxxxx
_____  18{          8888888888
8888b. l88,        ,88" ______
888888  18b,_    ,d88P  888888
888888b. `188bwwd88P' ,d888888
88888888b._ `"^^"'`.,d88888888
888888888888bo  od888888888888
88888888888888  88888888888888
88888888888888  88888888888888
"#,
        "alpine" => r#"
       .:::::::::::::::::::::.
      .:::::::::::::::::::::::.
     .:::::::::::::::::::::::::.
    .:::::::::::::::::::::::::::.
   .:::::::::,db,::::::::::::::::.
  .::::::::,d%%%%b,::,db,:::::::::.
 .:::::::,%%%%P'%%%b,d%%%b,::::::::.
.::::::,%%%%P,:::`%%%b'^q%%b,:::::::.
'::::,%%%%P,:::::::`%%%b:'^%%b,:::::'
 '::`%%%':'::::::::'q%%b'::`%%b'::'
  ':::::::::::::::::::::::::::::::'
   ':::::::::::::::::::::::::::::'
    ':::::::::::::::::::::::::::'
     ':::::::::::::::::::::::::'
      ':::::::::::::::::::::::'
       ':::::::::::::::::::::'
        "#,
        "zerene" => r#"
          ''
       .:kNK,
     ,dKWMMWd
   .lXMMMMMMK,
    .,lkKWMMWd.
        .;oONK,
            ':,...           ..,c,
             .l0X0Oo. ..,:ldOKNWMd
             '0MMMMX:.cONMMMMMMMWc
              ,oxdo:.  .;oONMMMMX;
            .l;            'cd0N0'
         .:xXWc               .,,
       ,oKWMMX;
    .cONMMMMM0'
     ,lkXWMMMk.
        .:d0Wd
           .,.
"#,
        "netbsd" => r#"
 _
 \\`-______,----__
  \\        __,---`_
   \\       `.____
    \\-______,----`-
     \\
      \\
       \\
        \\
         \\-
"#,
	"android" | "termux" => r#"
          -o         o-
          +hydNNNNdyh+
        +mMMMMMMMMMMMMm+
      `dMMm:NMMMMMMN:mMMd`
      hMMMMMMMMMMMMMMMMMMh
  ..  yyyyyyyyyyyyyyyyyyyy  ..
.mMMm`MMMMMMMMMMMMMMMMMMMM`mMMm.
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
-MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM-
 +yy+ MMMMMMMMMMMMMMMMMMMM +yy+
      mMMMMMMMMMMMMMMMMMMm
      `/++MMMMh++hMMMM++/`
          MMMMo  oMMMM
          MMMMo  oMMMM
          oNMm-  -mMNs
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
  0   `-'       `--' 
"#
    }
}

pub fn get_logo_color(name: &str) -> (u8, u8, u8) {
    let v = normalize(name);
    match v.as_str() {
        "arch" | "archlinux" | "artix" | "artixlinux" | "archlabs" => (106, 230, 255),
        "kyon" => (0, 219, 219),
        "omarchy" | "manjaro" => (152, 255, 97),
        "archcraft" => (131, 187, 173),
        "garuda" => (125, 104, 225),
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
        "pop" | "popos" | "pop_os" | "pop os" => (122, 225, 245),
        "elementary" | "elementary os" => (100, 186, 171),
        "void" | "voidlinux" => (71, 128, 97),
        "nixos" => (119, 179, 220),
        "mageia" => (47, 95, 143),
        "openmandriva" => (34, 129, 188),
        "gentoo" => (217, 200, 255),
        "lfs" => (255, 234, 174), 
        "bedrock" => (160, 160, 160), 
        "cachyos" => (3, 219, 209),
        "rfetch" => (100, 230, 255), 
        "mist" => (180, 218, 215), 
        "chimera" => (214, 80, 95),
        "alpine" => (13,89,127),
        "zerene" => (118, 75, 235),
        "netbsd" => (245, 152, 66),
        "android" => (163, 197, 78),
        _ => (255, 255, 255), 
    }
}

pub fn known_distros() -> Vec<&'static str> {
    vec![
        "arch", "debian", "ubuntu", "linuxmint", "kali", "raspbian",
        "fedora", "rhel", "centos", "rocky", "almalinux",
        "opensuse-tumbleweed", "opensuse-leap", "sles",
        "gentoo", "void", "nixos", "pop", "elementary", "mageia",
        "openmandriva", "lfs", "bedrock", "rfetch", "cachyos", "mist", "chimera", "zerene", "alpine",
        "android",
    ]
}

/// Hot-path replacement for `sysinfo::System`.
///
/// Constructing and refreshing a `System` costs ~0.4ms because sysinfo keeps
/// its own bookkeeping and parses far more of /proc than rfetch reads. We only
/// need the CPU brand plus four memory numbers, so parse /proc directly. The
/// derived values are identical to sysinfo's (verified: total = MemTotal,
/// used = MemTotal - MemAvailable, swap = SwapTotal - SwapFree).
pub struct SystemInfo {
    cpu_brand: String,
    mem_total_kb: u64,
    mem_available_kb: u64,
    swap_total_kb: u64,
    swap_free_kb: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        let (mem_total_kb, mem_available_kb, swap_total_kb, swap_free_kb) =
            read_meminfo().unwrap_or((0, 0, 0, 0));
        Self {
            cpu_brand: read_cpu_brand(),
            mem_total_kb,
            mem_available_kb,
            swap_total_kb,
            swap_free_kb,
        }
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

fn read_meminfo() -> Option<(u64, u64, u64, u64)> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    let (mut total, mut available, mut free, mut buffers, mut cached) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut swap_total, mut swap_free) = (0u64, 0u64);
    for line in content.lines() {
        let Some((key, rest)) = line.split_once(':') else { continue };
        let Some(value) = rest.split_whitespace().next().and_then(|v| v.parse::<u64>().ok()) else { continue };
        match key {
            "MemTotal" => total = value,
            "MemAvailable" => available = value,
            "MemFree" => free = value,
            "Buffers" => buffers = value,
            "Cached" => cached = value,
            "SwapTotal" => swap_total = value,
            "SwapFree" => swap_free = value,
            _ => {}
        }
    }
    if available == 0 {
        available = free + buffers + cached;
    }
    Some((total, available, swap_total, swap_free))
}

fn read_cpu_brand() -> String {
    let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") else {
        return String::new();
    };
    for line in content.lines() {
        let Some((key, value)) = line.split_once(':') else { continue };
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        match key.trim() {
            "model name" | "Processor" | "cpu model" | "Hardware" => return value.to_string(),
            _ => {}
        }
    }
    String::new()
}

pub fn cpu(info: &SystemInfo) -> String {
    info.cpu_brand.clone()
}

pub fn raw_os_id_or_name() -> String {
    os_id_or_name()
}


pub fn os() -> String {
    let v = os_id_or_name();
    format(&v).to_string()
}

pub fn ram_info(info: &SystemInfo) -> (String, String, String) {
    let used_bytes = info.mem_total_kb.saturating_sub(info.mem_available_kb) * 1024;
    let total_bytes = info.mem_total_kb * 1024;
    let used_gb = ((used_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let total_gb = ((total_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let pct = if total_gb > 0.0 {
        ((used_gb / total_gb) * 100.0).round() as u32
    } else {
        0
    };
    (used_gb.to_string(), total_gb.to_string(), pct.to_string())
}

pub struct DiskInfo {
    pub name: String,
    pub filesystem: String,
    pub used_gb: f64,
    pub total_gb: f64,
    pub usage_pct: f64,
}

const EXCLUDED_FS: &[&str] = &[
    "tmpfs", "devtmpfs", "squashfs", "overlay", "proc", "sysfs", "cgroup",
    "devpts", "hugetlbfs", "mqueue", "pstore", "securityfs", "efivarfs",
    "bpf", "tracefs", "debugfs", "configfs", "fusectl", "autofs",
];

/// `statvfs` reports the same block accounting `df` uses (`f_bfree`, not the
/// user-visible `f_bavail`), so used/total come out identical to `df -B1`.
fn statvfs_sizes(path: &str) -> Option<(u64, u64)> {
    use std::ffi::{CString, OsStr};
    use std::os::unix::ffi::OsStrExt;
    let c = CString::new(OsStr::new(path).as_bytes()).ok()?;
    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c.as_ptr(), &mut st) } != 0 {
        return None;
    }
    let frsize = if st.f_frsize != 0 { st.f_frsize } else { st.f_bsize } as u64;
    let total = (st.f_blocks as u64).checked_mul(frsize)?;
    let free = (st.f_bfree as u64).checked_mul(frsize)?;
    Some((total, total.checked_sub(free)?))
}

pub fn disks_info() -> Vec<DiskInfo> {
    let gb = 1024.0 * 1024.0 * 1024.0;
    let mut result = Vec::new();

    // Read the mount table directly and statvfs each real device. This is what
    // `df` does, minus the ~0.9ms process spawn, and yields the same numbers.
    if let Ok(mounts) = std::fs::read_to_string("/proc/mounts") {
        let mut seen = HashSet::new();
        for line in mounts.lines() {
            let mut parts = line.split_whitespace();
            let source = match parts.next() { Some(v) => v, None => continue };
            let target = match parts.next() { Some(v) => v, None => continue };
            let fstype = match parts.next() { Some(v) => v, None => continue };

            if !source.starts_with("/dev/") || target.starts_with("/boot") {
                continue;
            }
            if fstype.starts_with("fuse.") || fstype == "fuse" || EXCLUDED_FS.contains(&fstype) {
                continue;
            }

            let (total_bytes, used_bytes) = match statvfs_sizes(target) {
                Some(v) => v,
                None => continue,
            };
            if total_bytes == 0 {
                continue;
            }

            let name = source.strip_prefix("/dev/").unwrap_or(source).to_string();
            if !seen.insert(name.clone()) {
                continue;
            }

            let total_gb = total_bytes as f64 / gb;
            let used_gb = used_bytes as f64 / gb;
            let pct = (used_bytes as f64 / total_bytes as f64) * 100.0;

            result.push(DiskInfo {
                name,
                filesystem: fstype.to_string(),
                used_gb: (used_gb * 10.0).round() / 10.0,
                total_gb: (total_gb * 10.0).round() / 10.0,
                usage_pct: (pct * 10.0).round() / 10.0,
            });
        }

        if !result.is_empty() {
            return result;
        }
    }

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let mut seen = HashSet::new();
    for disk in disks.list() {
        let fs_name = disk.file_system().to_string_lossy();
        if fs_name == "fuse" || fs_name.starts_with("fuse.") {
            continue;
        }
        if fs_name == "fusectl" {
            continue;
        }
        let skip_fs: &[&str] = &[
            "tmpfs", "devtmpfs", "squashfs", "overlay", "proc", "sysfs",
            "cgroup", "devpts", "hugetlbfs", "mqueue", "pstore",
            "securityfs", "efivarfs", "bpf", "tracefs", "debugfs",
            "configfs", "autofs", "efiivarfs",
        ];
        if skip_fs.contains(&fs_name.as_ref()) {
            continue;
        }

        let mountpoint = disk.mount_point().to_string_lossy().to_string();
        if mountpoint.starts_with("/boot") {
            continue;
        }

        let total = disk.total_space() as f64 / gb;
        let avail = disk.available_space() as f64 / gb;
        let used = total - avail;
        let pct = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };

        let dev_name = disk.name().to_string_lossy().to_string();
        let name = if dev_name.starts_with('/') {
            dev_name.strip_prefix("/dev/").unwrap_or(&dev_name)
        } else {
            &dev_name
        };

        let fstype = disk.file_system().to_string_lossy().to_string();

        let dev_name = name.to_string();
        if !seen.insert(dev_name.clone()) {
            continue;
        }
        result.push(DiskInfo {
            name: dev_name,
            filesystem: fstype,
            used_gb: (used * 10.0).round() / 10.0,
            total_gb: (total * 10.0).round() / 10.0,
            usage_pct: (pct * 10.0).round() / 10.0,
        });
    }

    result
}

pub fn wmde() -> String {
	let unp_de = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "rfetch".to_string());
	match unp_de.to_lowercase().as_str() {
		"gnome" => String::from(" gnome"),
		"kde" | "plasma" => String::from(" kde"),
		"niri" => String::from(" niri"),
		"hyprland" => String::from(" hypr"),
		"xfce" => String::from(" xfce"),
		"sway" => String::from(" sway"),
		"i3" => String::from(" i3"),
		"mango" | "mangowm" => String::from("󱁆 mango"),
		"cinnamon" | "x-cinnamon" => String::from(" cinnamon"),
		_ => format!(" {}", unp_de.to_lowercase())
	}
}
pub fn kernel() -> String {
    if is_termux() {
        if let Ok(ver) = std::fs::read_to_string("/proc/version") {
            let version = ver.split_whitespace().nth(2).unwrap_or("unknown");
            return format!("󰌽 linux {}", version);
        }
    }
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| format!("󰌽 linux {}", s.trim()))
        .unwrap_or_else(|_| {
            let output = Command::new("uname").arg("-sr").output().expect("");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        })
}
pub fn shell() -> String {
    let mut sh_unp = String::from("");
    if let Ok(shell_path) = std::env::var("SHELL") {
        if let Some(name) = Path::new(&shell_path).file_name() {
            sh_unp = name.to_string_lossy().to_string();
        }
    };

    if let Ok(passwd) = std::fs::read_to_string("/etc/passwd") {
        let username = std::env::var("USER").unwrap_or_default();
        for line in passwd.lines() {
            if line.starts_with(&format!("{}:", username)) {
                if let Some(shell) = line.split(':').last() {
                    sh_unp = Path::new(shell)
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                }
            }
        }
    };
    if sh_unp.trim().is_empty() {
        sh_unp = String::from("unknown");
    };
    match sh_unp.as_str() {
        "fish" => " fish".to_string(),
        "bash" => " bash".to_string(),
        "zsh" => " zsh".to_string(),
        "sh" => " sh".to_string(),
        _ => " unknown".to_string()
    }
}

pub fn terminal() -> String {
    let mut unp_t = String::new();
    if let Some(term_program) = std::env::var("TERM_PROGRAM")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        unp_t = term_program;
    }

    if let Some(term) = std::env::var("TERM")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        unp_t = term;
    }
    if !unp_t.is_empty() {
        return match unp_t.to_lowercase().as_str() {
            "alacritty" => "󱐋 alacritty".to_string(),
            "xterm-kitty" => " kitty".to_string(),
            "tabby" => " tabby".to_string(),
            "foot" => " foot".to_string(),
            "xterm-256color" => " DE terminal".to_string(),
            "xterm-ghostty" => "󰊠 ghostty".to_string(),
            _ => format!(" {}", unp_t.to_lowercase())
        }
    }

    String::new()
}
pub fn gpu() -> String {
    if let Some(name) = gpu_from_sysfs() {
        return name;
    }

    if is_termux() {
        if let Ok(soc) = std::fs::read_to_string("/sys/devices/soc0/soc_id") {
            let id = soc.trim().to_string();
            return format!("SoC (id: {})", id);
        }
        if let Ok(hw) = std::fs::read_to_string("/sys/devices/soc0/machine") {
            return hw.trim().to_string();
        }
    }

    "none found, maybe integrated".to_string()
}

fn read_hex(path: &str) -> Option<u32> {
    let s = std::fs::read_to_string(path).ok()?;
    let s = s.trim();
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u32::from_str_radix(s, 16).ok()
}

// the marketing name comes from the same amdgpu.ids table libdrm used, keyed by
// PCI (device, revision) id, so we get the exact same string without linking
// libdrm (which every process otherwise had to load at startup).
fn amdgpu_name(device_id: u32, revision_id: u32) -> Option<String> {
    let ids = std::fs::read_to_string("/usr/share/libdrm/amdgpu.ids").ok()?;
    for line in ids.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split(',');
        let (did, rid) = match (fields.next(), fields.next()) {
            (Some(d), Some(r)) => (d.trim(), r.trim()),
            _ => continue,
        };
        let name = fields.collect::<Vec<_>>().join(",");
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        if let (Ok(d), Ok(r)) = (u32::from_str_radix(did, 16), u32::from_str_radix(rid, 16)) {
            if d == device_id && r == revision_id {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn nvidia_name() -> Option<String> {
    for e in std::fs::read_dir("/proc/driver/nvidia/gpus").ok()?.flatten() {
        let info = match std::fs::read_to_string(e.path().join("information")) {
            Ok(i) => i,
            Err(_) => continue,
        };
        for line in info.lines() {
            if let Some(rest) = line.strip_prefix("Model:") {
                let name = rest.trim();
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

// mirrors gfxinfo's order (amd then nvidia) and the names it produced, but
// straight from sysfs/proc so there is nothing to dlopen or ioctl.
fn gpu_from_sysfs() -> Option<String> {
    let mut amd = None;
    let mut nvidia = None;
    for e in std::fs::read_dir("/sys/class/drm").ok()?.flatten() {
        let name = e.file_name();
        let name = name.to_string_lossy();
        let rest = match name.strip_prefix("card") {
            Some(r) => r,
            None => continue,
        };
        // only "cardN" nodes, skip connectors like card0-DP-1
        if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let dev = format!("/sys/class/drm/{}/device", name);
        match read_hex(&format!("{}/vendor", dev)) {
            Some(0x1002) if amd.is_none() => {
                if let (Some(did), Some(rid)) =
                    (read_hex(&format!("{}/device", dev)), read_hex(&format!("{}/revision", dev)))
                {
                    // fall back to libdrm's default when the id table misses
                    amd = amdgpu_name(did, rid).or_else(|| Some("AMD Radeon Graphics".to_string()));
                }
            }
            Some(0x10de) if nvidia.is_none() => {
                nvidia = nvidia_name();
            }
            _ => {}
        }
    }
    amd.or(nvidia)
}

pub fn hostusr() -> String {
    format!(
        "{} ( {} )",
        whoami::username(),
        whoami::fallible::hostname().unwrap_or_default()
    )
}

pub fn uptime() -> String {
    std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|content| {
            let secs: f64 = content.split_whitespace().next()?.parse().ok()?;
            let total_secs = secs as u64;
            let days = total_secs / 86400;
            let hours = (total_secs % 86400) / 3600;
            let mins = (total_secs % 3600) / 60;
            let mut parts = Vec::new();
            if days > 0 {
                parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
            }
            if hours > 0 {
                parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
            }
            if mins > 0 || parts.is_empty() {
                parts.push(format!("{} minute{}", mins, if mins == 1 { "" } else { "s" }));
            }
            Some(parts.join(", "))
        })
        .unwrap_or_else(|| {
            let output = Command::new("uptime").arg("-p").output().expect("");
            String::from_utf8_lossy(&output.stdout).trim().to_string().replace("up ", "")
        })
}

/// Reads charge from sysfs directly. `starship_battery::Manager` costs ~0.9ms
/// (walks the whole power_supply class, builds typed units) but ends up reading
/// these same `capacity` nodes, so the value is identical.
fn read_battery_percent() -> Option<usize> {
    if let Ok(cap) = std::fs::read_to_string("/sys/class/power_supply/battery/capacity") {
        if let Ok(pct) = cap.trim().parse::<usize>() {
            return Some(pct);
        }
    }
    let entries = std::fs::read_dir("/sys/class/power_supply").ok()?;
    for e in entries.flatten() {
        let dir = e.path();
        let ty = match std::fs::read_to_string(dir.join("type")) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ty.trim() != "Battery" {
            continue;
        }
        if let Ok(cap) = std::fs::read_to_string(dir.join("capacity")) {
            if let Ok(pct) = cap.trim().parse::<usize>() {
                return Some(pct);
            }
        }
    }
    None
}

pub fn get_battery_charge() -> usize {
    if let Some(pct) = read_battery_percent() {
        return pct;
    }

    // sysfs exists on linux, so an empty scan means this machine genuinely has
    // no battery. Bail instead of paying ~0.9ms for a manager walk.
    if Path::new("/sys/class/power_supply").exists() {
        return 500;
    }

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

// --- os age (time since install) ---

use std::time::{SystemTime, UNIX_EPOCH};

fn birth_time_of(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).ok()?.created().ok()
}

fn modified_time_of(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).ok()?.modified().ok()
}

fn stat_birth_fallback(path: &str) -> Option<SystemTime> {
    // `stat -c %W` prints birth as unix secs, 0 if unknown.
    let out = Command::new("stat").arg("-c").arg("%W").arg(path).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let secs: u64 = s.parse().ok()?;
    if secs == 0 {
        return None;
    }
    UNIX_EPOCH.checked_add(std::time::Duration::from_secs(secs))
}

/// Try to parse the first timestamp in /var/log/pacman.log (arch btw).
/// Lines look like: [2026-09-07T20:47:00+0200] [PACMAN] Running ...
fn pacman_log_install_time() -> Option<SystemTime> {
    let content = std::fs::read_to_string("/var/log/pacman.log").ok()?;
    for line in content.lines() {
        let line = line.trim();
        if !line.starts_with('[') || line.len() < 11 {
            continue;
        }
        // extract YYYY-MM-DD inside brackets
        let end = line.find(']')?;
        let inner = &line[1..end];
        // inner is like 2026-09-07T20:47:00+0200, take date part
        if inner.len() < 10 {
            continue;
        }
        let date_part = &inner[0..10];
        let mut parts = date_part.split('-');
        let y: i32 = parts.next()?.parse().ok()?;
        let m: u32 = parts.next()?.parse().ok()?;
        let d: u32 = parts.next()?.parse().ok()?;
        if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
            continue;
        }
        let days = days_from_civil(y, m, d);
        let secs = days * 86400;
        if secs < 0 {
            continue;
        }
        return UNIX_EPOCH.checked_add(std::time::Duration::from_secs(secs as u64));
    }
    None
}

fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    // Howard Hinnant's days_from_civil, days since 1970-01-01
    let y = if m <= 2 { y - 1 } else { y } as i64;
    let m = m as i64;
    let d = d as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    // Howard Hinnant's civil_from_days, inverse of above
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 {
        y += 1;
    }
    (y as i32, m as u32, d as u32)
}

fn format_install_date(t: SystemTime) -> Option<String> {
    let secs = t.duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let days = secs.div_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    Some(format!("{:04}-{:02}-{:02}", y, m, d))
}

fn format_age_duration(secs: u64) -> String {
    let days = secs / 86400;
    if days == 0 {
        let hours = secs / 3600;
        if hours == 0 {
            let mins = secs / 60;
            if mins <= 1 {
                return "just now".to_string();
            }
            return format!("{} minutes", mins);
        }
        if hours == 1 {
            return "1 hour".to_string();
        }
        return format!("{} hours", hours);
    }
    if days == 1 {
        return "1 day".to_string();
    }
    if days < 30 {
        return format!("{} days", days);
    }
    if days < 365 {
        let months = days / 30;
        let rem = days % 30;
        let m_word = if months == 1 { "month" } else { "months" };
        if rem == 0 {
            return format!("{} {}", months, m_word);
        }
        let d_word = if rem == 1 { "day" } else { "days" };
        return format!("{} {} {} {}", months, m_word, rem, d_word);
    }
    let years = days / 365;
    let rem = days % 365;
    let months = rem / 30;
    let d = rem % 30;
    let y_word = if years == 1 { "year" } else { "years" };
    let mut s = format!("{} {}", years, y_word);
    if months > 0 {
        s.push_str(&format!(" {} {}", months, if months == 1 { "month" } else { "months" }));
    }
    if d > 0 {
        s.push_str(&format!(" {} {}", d, if d == 1 { "day" } else { "days" }));
    }
    s
}

pub fn os_install_time() -> Option<SystemTime> {
    let mut candidates: Vec<SystemTime> = Vec::new();

    // 1. birth time of / (filesystem creation ~= install). best generic signal.
    if let Some(t) = birth_time_of("/").or_else(|| stat_birth_fallback("/")) {
        candidates.push(t);
    }
    // 2. birth of machine-id files (generated at install)
    for p in ["/etc/machine-id", "/var/lib/dbus/machine-id"] {
        if let Some(t) = birth_time_of(p).or_else(|| stat_birth_fallback(p)) {
            candidates.push(t);
        }
    }
    // 3. birth of pacman log (arch)
    if let Some(t) = birth_time_of("/var/log/pacman.log") {
        candidates.push(t);
    }

    if !candidates.is_empty() {
        // oldest = most likely the install
        candidates.sort();
        return candidates.into_iter().next();
    }

    // 4. fallbacks when btime is unsupported (some fs/kernels return 0/err):
    // mtime of write-once-at-install files
    for p in [
        "/etc/machine-id",
        "/var/lib/dbus/machine-id",
        "/root/anaconda-ks.cfg",
        "/var/log/installer/syslog",
        "/etc/arch-release",
    ] {
        if let Some(t) = modified_time_of(p) {
            candidates.push(t);
        }
    }
    // 5. parse pacman log content (arch btw^2)
    if let Some(t) = pacman_log_install_time() {
        candidates.push(t);
    }

    if candidates.is_empty() {
        return None;
    }
    candidates.sort();
    candidates.into_iter().next()
}

pub fn os_age() -> String {
    let install = match os_install_time() {
        Some(t) => t,
        None => return "unknown".to_string(),
    };
    let now = SystemTime::now();
    let dur = match now.duration_since(install) {
        Ok(d) => d,
        Err(_) => return "unknown".to_string(),
    };
    let age = format_age_duration(dur.as_secs());
    match format_install_date(install) {
        Some(date) => format!("{} (installed {})", age, date),
        None => age,
    }
}

// --- promoted-to-stable hardware gaps: swap / load / processes / boot ---

pub fn swap_info(info: &SystemInfo) -> (String, String, String) {
    let used_bytes = info.swap_total_kb.saturating_sub(info.swap_free_kb) * 1024;
    let total_bytes = info.swap_total_kb * 1024;
    let used_gb = ((used_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let total_gb = ((total_bytes as f32 / 1024.0 / 1024.0 / 1024.0) * 10.0).ceil() / 10.0;
    let pct = if total_gb > 0.0 {
        ((used_gb / total_gb) * 100.0).round() as u32
    } else {
        0
    };
    (used_gb.to_string(), total_gb.to_string(), pct.to_string())
}

pub fn load_avg() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
        let mut parts = content.split_whitespace();
        if let (Some(one), Some(five), Some(fifteen)) = (parts.next(), parts.next(), parts.next()) {
            return format!("{}, {}, {}", one, five, fifteen);
        }
    }
    let avg = System::load_average();
    format!("{:.2}, {:.2}, {:.2}", avg.one, avg.five, avg.fifteen)
}

pub fn process_count() -> Option<usize> {
    // counting /proc numeric dirs = processes (threads excluded, unlike loadavg total).
    // getdents64 directly so we skip the OsString read_dir allocates per entry,
    // which is one heap allocation for every pid on every run.
    if let Some(n) = count_proc_pids() {
        if n > 0 {
            return Some(n);
        }
    }
    // fallback: total from /proc/loadavg 4th field ("3/786" -> 786, threads included)
    if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
        let fourth = content.split_whitespace().nth(3).unwrap_or("");
        if let Some(total) = fourth.split('/').nth(1) {
            if let Ok(v) = total.parse::<usize>() {
                return Some(v);
            }
        }
    }
    None
}

fn count_proc_pids() -> Option<usize> {
    use std::os::unix::io::AsRawFd;
    let dir = std::fs::File::open("/proc").ok()?;
    let fd = dir.as_raw_fd();
    let mut buf = [0u8; 16384];
    let mut n = 0usize;
    loop {
        let nread = unsafe {
            libc::syscall(
                libc::SYS_getdents64,
                fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len(),
            )
        };
        if nread <= 0 {
            break;
        }
        let end = nread as usize;
        let mut off = 0usize;
        while off + 19 <= end {
            let d = unsafe { &*(buf.as_ptr().add(off) as *const libc::dirent64) };
            let reclen = d.d_reclen as usize;
            if reclen == 0 || off + reclen > end {
                break;
            }
            let name = d.d_name.as_ptr() as *const u8;
            let mut is_pid = false;
            let mut i = 0usize;
            loop {
                let c = unsafe { *name.add(i) };
                if c == 0 {
                    break;
                }
                if !c.is_ascii_digit() {
                    is_pid = false;
                    break;
                }
                is_pid = true;
                i += 1;
                if i >= 256 {
                    is_pid = false;
                    break;
                }
            }
            if is_pid {
                n += 1;
            }
            off += reclen;
        }
    }
    Some(n)
}

fn read_btime() -> Option<u64> {
    let content = std::fs::read_to_string("/proc/stat").ok()?;
    content
        .lines()
        .find_map(|l| l.strip_prefix("btime "))
        .and_then(|v| v.trim().parse().ok())
}

/// Formats an epoch timestamp in local time via `localtime_r` so we match what
/// `uptime -s` printed, without paying for a process spawn (~1ms).
fn format_local(ts: u64) -> Option<String> {
    let t = ts as libc::time_t;
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    if unsafe { libc::localtime_r(&t, &mut tm) }.is_null() {
        return None;
    }
    Some(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec
    ))
}

pub fn boot_time() -> String {
    if let Some(ts) = read_btime() {
        if let Some(s) = format_local(ts) {
            return s;
        }
    }
    // fallback: sysinfo boot timestamp formatted via `date` (local tz)
    let ts = System::boot_time();
    if ts > 0 {
        if let Ok(out) = Command::new("date")
            .arg("-d")
            .arg(format!("@{}", ts))
            .arg("+%F %T")
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    "unknown".to_string()
}

