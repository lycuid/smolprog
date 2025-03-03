package logger

import (
	"fmt"
	"os"
	. "smolprog/utils"
	"strings"
	"time"
)

const NET_DIR = "/sys/class/net"

type networkBytes struct{ rx, tx float64 }

func getActiveInterface() (user_iface string) {
	// prioritizing user selected interface.
	user_iface, _ = FirstLineOf(XDG_RUNTIME_DIR + "/net/active")

	active_ifaces := getNetworkInterfaces()
	for i, j := 0, 0; i < len(active_ifaces); i++ {
		if isInterfaceActive(active_ifaces[i]) {
			active_ifaces[j], j = active_ifaces[i], j+1
		}
	}
	// no interfaces active at this point.
	if len(active_ifaces) == 0 {
		return ""
	}

	var isUserIfaceActive bool
	for _, iface := range active_ifaces {
		if isUserIfaceActive = iface == user_iface; isUserIfaceActive {
			break
		}
	}
	if isUserIfaceActive {
		return user_iface
	}
	return active_ifaces[0]
}

func isInterfaceActive(iface string) bool {
	line, err := FirstLineOf(NET_DIR + "/" + iface + "/operstate")
	return err == nil && strings.TrimSpace(line) == "up"
}

func getNetworkInterfaces() (ifaces []string) {
	if entries, err := os.ReadDir(NET_DIR); err == nil {
		ifaces = make([]string, len(entries))
		for _, entry := range entries {
			ifaces = append(ifaces, entry.Name())
		}
	}
	return ifaces
}

func getNetworkBytes(iface string) (bytes networkBytes) {
	if rx_string, err := FirstLineOf(NET_DIR + "/" + iface + "/statistics/rx_bytes"); err == nil {
		bytes.rx = Float(rx_string)
	}
	if tx_string, err := FirstLineOf(NET_DIR + "/" + iface + "/statistics/tx_bytes"); err == nil {
		bytes.tx = Float(tx_string)
	}
	return bytes
}

type Network struct {
	networkBytes
	iface string
}

func (net *Network) Run(slot int, channel chan<- *Message) {
	IntervalRunner(net, slot, channel)
}

func (_ *Network) Interval() time.Duration {
	return time.Second
}

func (net *Network) Value() string {
	return "<BtnL=xdotool key super+ctrl+n>  " + net.calculate() + "  </BtnL>"
}

func (net *Network) calculate() string {
	var (
		iface  string
		rx, tx = net.rx, net.tx // old values
	)

	if iface = getActiveInterface(); len(iface) == 0 {
		goto DEFAULT
	}

	if bytes := getNetworkBytes(iface); true {
		// if the interface has changed, then the difference (that we check
		// later) between the old and new values would be huge, to avoid that we
		// set the old values equal to the new value.
		if net.rx, net.tx = bytes.rx, bytes.tx; iface != net.iface {
			rx, tx, net.iface = net.rx, net.tx, iface
		}
	}
	rx, tx = (net.rx-rx)/1024., (net.tx-tx)/1024.

	return fmt.Sprintf("%s:  %.2f kib/s  %.2f kib/s", iface, rx, tx)

DEFAULT:
	return "net: ?"
}
