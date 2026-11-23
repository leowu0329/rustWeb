
1. 準備乾淨的正式部署資料夾
建議在 C 槽建立一個固定的部署目錄（避免放在桌面或暫存資料夾），將編譯好的執行檔與 .env 放入其中：
一次執行完整建立指令
如果想一次完成，可以直接複製：
=======================================================================
mkdir -p /c/RyowuTestCode/RustWeb/logs

cp /c/RyowuTestCode/rusttest/rust_web/target/release/rust_web.exe \
   /c/RyowuTestCode/RustWeb/rust_web.exe

cat > /c/RyowuTestCode/RustWeb/.env <<'EOF'
DATABASE_URL=sqlite://guestbook.db
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
EOF

echo "部署目錄建立完成！"

ls -la /c/RyowuTestCode/RustWeb
=======================================================================

就會產生下列的目錄檔案
C:\RyowuTestCode\RustWeb\
├── rust_web.exe        <-- 從專案的 target\release\rust_web.exe 複製過來
├── .env                <-- 設定 DATABASE_URL、SERVER_HOST、SERVER_PORT
└── logs\               <-- 手動建立此空資料夾，用來存放網站執行日誌
(確認 .env 內容：若希望區域網路內其他電腦也能連線，可將 SERVER_HOST 設為 0.0.0.0；若僅限本機則維持 127.0.0.1。)

2. 安裝 NSSM 服務管理工具
以系統管理員身分開啟 PowerShell，使用 Windows 內建的 winget 套件管理器一鍵安裝 NSSM：
winget install NSSM.NSSM
(安裝完成後，請重新開啟一個系統管理員身分的 PowerShell 視窗，讓 nssm 指令的環境變數生效。)

3. 註冊並配置 RustWebService 背景服務
在系統管理員身分的 PowerShell 中，依序執行以下指令註冊服務（這裡特別指定了 AppDirectory 工作目錄，確保程式能正確讀取同資料夾下的 .env 與建立 guestbook.db，不會誤建到 C:\Windows\System32）：

PowerShell
# 1. 安裝服務並指定執行檔路徑（服務名稱自訂為 RustWebService）
nssm install RustWebService "C:\RyowuTestCode\RustWeb\rust_web.exe"

# 2. 【關鍵設定】指定程式啟動時的工作目錄（必須與 .env 所在目錄一致）
nssm set RustWebService AppDirectory "C:\RyowuTestCode\RustWeb"

# 3. 設定服務顯示名稱與描述（方便在 Windows「服務」清單中辨識）
nssm set RustWebService DisplayName "Rust Web 留言板系統"
nssm set RustWebService Description "由 Rust + Axum + SQLite 驅動的背景網站服務"

# 4. 設定開機自動啟動
nssm set RustWebService Start SERVICE_AUTO_START

# 5. 將終端機標準輸出與錯誤訊息自動導向日誌檔（方便日後排查問題）
nssm set RustWebService AppStdout "C:\RyowuTestCode\RustWeb\logs\output.log"
nssm set RustWebService AppStderr "C:\RyowuTestCode\RustWeb\logs\error.log"

4. 啟動服務與日常管理指令
設定完成後，透過以下指令啟動並管理你的背景服務：

PowerShell
# 1. 立即啟動背景服務
nssm start RustWebService

# 2. 檢查服務目前運行狀態（顯示 SERVICE_RUNNING 即代表成功）
nssm status RustWebService
此時不需要開啟任何終端機視窗，直接打開瀏覽器前往 [http://127.0.0.1:3000](http://127.0.0.1:3000) 就能看到網站正在背景穩定運作，即使重新開機也會自動在背景啟動。

日後常用維護指令（需以系統管理員身分執行）：
更新網站程式版本：
nssm stop RustWebService
# 將新編譯的 rust_web.exe 覆蓋至 C:\RyowuTestCode\RustWeb\rust_web.exe
nssm start RustWebService


重啟服務：nssm restart RustWebService
開啟圖形化設定介面微調參數：nssm edit RustWebService
徹底移除此背景服務：
nssm stop RustWebService
nssm remove RustWebService confirm
