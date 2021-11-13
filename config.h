#include <blocks.h>

#define SEPERATOR_COLOR "#171717"
#define SEPERATOR SEPERATOR_COLOR ":2"
#define BORDER "#089CAC"

static Block blks[] = {
    {.runner = blk_net,
     .def = "No Internet.",
     .fmt = "<BtnL=xdotool key super+ctrl+n> %s  </BtnL><BoxL=" SEPERATOR
            "> </BoxL>"},
    {.runner = blk_cpu,
     .def = "cpu: ?",
     .fmt = "<BtnL=notify_max_cpu> %s  </BtnL><BoxL=" SEPERATOR "> </BoxL>"},
    {.runner = blk_mem,
     .def = "mem: ?",
     .fmt = "<BtnL=notify_max_mem> %s  </BtnL><BoxL=" SEPERATOR "> </BoxL>"},
    {.pipefile = pipe_vol,
     .def = "vol: ?",
     .fmt = "<ScrlU:Shift=volume 5%+><ScrlD:Shift=volume "
            "5%-><ScrlU=volume 1%+><ScrlD=volume "
            "1%-><BtnL=volume toggle> "
            "%s  </BtnL></ScrlD></ScrlU></ScrlD></ScrlU><BoxL=" SEPERATOR
            "> </BoxL>"},
    {.runner = blk_bat, .def = "battery: ?", .fmt = " %s  "},
    {.runner = blk_dat,
     .def = "date: ? ",
     .fmt = "<BoxT=" BORDER "><BoxB=" BORDER "><BoxL=" BORDER "><BoxR=" BORDER
            "><Bg=" SEPERATOR_COLOR "> %s </Bg></BoxR></BoxL></BoxB></BoxT>"}};
