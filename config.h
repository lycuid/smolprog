#include <modules.h>

#define BORDER "#089CAC"
#define SEPERATOR_COLOR "#171717"

#define SEPERATOR "<Box:Left=" SEPERATOR_COLOR ":2> </Box>"

static Module mods[] = {
    {.runner = module_network,
     .def = "No Internet.",
     .fmt = "<BtnL=xdotool key super+ctrl+n> %s  </BtnL>" SEPERATOR},
    {.runner = module_cpu,
     .def = "cpu: ?",
     .fmt = "<BtnL=notify_max_cpu> %s  </BtnL>" SEPERATOR},
    {.runner = module_memory,
     .def = "mem: ?",
     .fmt = "<BtnL=notify_max_mem> %s  </BtnL>" SEPERATOR},
    {.runner = module_volume,
     .runnertype = FifoRunner,
     .def = "vol: ?",
     .fmt = "<ScrlU:Shift=volume 5%+><ScrlD:Shift=volume "
            "5%-><ScrlU=volume 1%+><ScrlD=volume "
            "1%-><BtnL=volume toggle> "
            "%s  </BtnL></ScrlD></ScrlU></ScrlD></ScrlU>" SEPERATOR},
    {.runner = module_tmuxls,
     .def = "sessions: ?",
     .fmt = "<BtnL=notify_tmux_ls> %s  </BtnL>" SEPERATOR},
    {.runner = module_battery, .def = "battery: ?", .fmt = " %s  "},
    {.runner = module_date,
     .def = "date: ? ",
     .fmt = "<Box:Top=" BORDER "><Box:Bottom=" BORDER "><Box:Left=" BORDER
            "><Box:Right=" BORDER "><Bg=" SEPERATOR_COLOR
            "><Fn=1> %s </Fn></Bg></Box></Box></Box></Box>"}};
