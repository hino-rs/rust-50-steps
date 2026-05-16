$projects = [ordered]@{
    "level1" = @("fizzbuzz", "guessing_game", "cli_calculator", "string_reverser", "temp_converter", "password_generator", "multiplication_table", "unit_converter", "anagram_checker", "dice_simulator")
    "level2" = @("cli_todo", "text_counter", "csv_parser", "cli_stopwatch", "grade_calculator", "bmi_logger", "file_lister", "calendar_cli", "morse_converter", "config_parser")
    "level3" = @("weather_cli", "json_formatter", "base64_tool", "grep_clone", "port_scanner", "pomodoro_timer", "sqlite_address_book", "url_shortener", "web_scraper", "snake_game")
    "level4" = @("tcp_chat", "http_api", "tui_file_manager", "parallel_resizer", "async_rss_reader", "kv_store", "tui_notepad", "backup_daemon", "rate_limiter", "markdown_viewer")
    "level5" = @("lang_interpreter", "web_router", "mini_rdbms", "virtual_machine", "lisp_interpreter", "http_proxy", "packet_sniffer", "crypto_tool", "p2p_node", "container_runtime")
}

$index = 1

foreach ($level in $projects.Keys) {
    foreach ($name in $projects[$level]) {
        $prefix = "{0:D2}" -f $index
        $dirName = "$level\$prefix`_$name"
        
        Write-Host "Creating: $dirName" -ForegroundColor Cyan
        
        cargo new $dirName --name $name
        
        $index++
    }
}

Write-Host ""