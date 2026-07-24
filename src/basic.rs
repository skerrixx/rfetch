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
        "chimera" => "󱗽 chimera", // yes
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
"#
    }
}

pub fn get_logo_color(name: &str) -> (u8, u8, u8) {
    let v = normalize(name);
    match v.as_str() {
        "arch" | "archlinux" => (106, 230, 255),
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
        _ => (255, 255, 255), 
    }
}

pub fn known_distros() -> Vec<&'static str> {
    vec![
        "arch", "debian", "ubuntu", "linuxmint", "kali", "raspbian",
        "fedora", "rhel", "centos", "rocky", "almalinux",
        "opensuse-tumbleweed", "opensuse-leap", "sles",
        "gentoo", "void", "nixos", "pop", "elementary", "mageia",
        "openmandriva", "lfs", "bedrock", "rfetch", "cachyos", "mist", "chimera"
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

pub struct DiskInfo {
    pub name: String,
    pub filesystem: String,
    pub mount_point: String,
    pub used_gb: f64,
    pub total_gb: f64,
    pub usage_pct: f64,
}

pub fn disks_info() -> Vec<DiskInfo> {
    let gb = 1024.0 * 1024.0 * 1024.0;
    let mut result = Vec::new();

    let df_output = Command::new("df")
        .arg("-B1")
        .arg("--exclude-type=tmpfs")
        .arg("--exclude-type=devtmpfs")
        .arg("--exclude-type=squashfs")
        .arg("--exclude-type=overlay")
        .arg("--exclude-type=proc")
        .arg("--exclude-type=sysfs")
        .arg("--exclude-type=cgroup")
        .arg("--exclude-type=devpts")
        .arg("--exclude-type=hugetlbfs")
        .arg("--exclude-type=mqueue")
        .arg("--exclude-type=pstore")
        .arg("--exclude-type=securityfs")
        .arg("--exclude-type=efivarfs")
        .arg("--exclude-type=bpf")
        .arg("--exclude-type=tracefs")
        .arg("--exclude-type=debugfs")
        .arg("--exclude-type=configfs")
        .arg("--exclude-type=fusectl")
        .arg("--exclude-type=autofs")
        .arg("--output=source,fstype,target,size,used,avail")
        .output()
        .ok();

    if let Some(output) = df_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut lines = stdout.lines();
            lines.next(); // skip header
            for line in lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    continue;
                }

                let source = parts[0];
                let fstype = parts[1];
                let target = parts[2];

                // Skip pseudo-filesystems and /boot
                if target.starts_with("/boot") {
                    continue;
                }
                // Skip if source doesn't look like a real device (/dev/...)
                if !source.starts_with("/dev/") {
                    continue;
                }

                let total_bytes: f64 = match parts[3].parse() { Ok(v) => v, Err(_) => continue };
                let used_bytes: f64 = match parts[4].parse() { Ok(v) => v, Err(_) => continue };

                if total_bytes <= 0.0 {
                    continue;
                }

                let total_gb = total_bytes / gb;
                let used_gb = used_bytes / gb;
                let pct = (used_bytes / total_bytes) * 100.0;


                let name = source.strip_prefix("/dev/").unwrap_or(source).to_string();

                result.push(DiskInfo {
                    name,
                    filesystem: fstype.to_string(),
                    mount_point: target.to_string(),
                    used_gb: (used_gb * 10.0).round() / 10.0,
                    total_gb: (total_gb * 10.0).round() / 10.0,
                    usage_pct: (pct * 10.0).round() / 10.0,
                });
            }

            if !result.is_empty() {
                return result;
            }
        }
    }

    let disks = sysinfo::Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let fs_name = disk.file_system().to_string_lossy();
        let skip_fs: &[&str] = &[
            "tmpfs", "devtmpfs", "squashfs", "overlay", "proc", "sysfs",
            "cgroup", "devpts", "hugetlbfs", "mqueue", "pstore",
            "securityfs", "efivarfs", "bpf", "tracefs", "debugfs",
            "configfs", "fuse", "fusectl", "autofs", "efiivarfs",
        ];
        if skip_fs.contains(&fs_name.as_ref()) {
            continue;
        }

        let mount_point = disk.mount_point().to_string_lossy().to_string();
        if mount_point.starts_with("/boot") {
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

        result.push(DiskInfo {
            name: name.to_string(),
            filesystem: fstype,
            mount_point: mount_point.clone(),
            used_gb: (used * 10.0).round() / 10.0,
            total_gb: (total * 10.0).round() / 10.0,
            usage_pct: (pct * 10.0).round() / 10.0,
        });
    }

    result
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

