use std::process::Command;

pub fn send_new_version_notification(title: String, body: String) {
    // 使用 Windows 7 兼容的备用方案
    send_notification_with_url_fallback(&title, &body, "https://snowshot.top/");
}

// 备用方案：使用简单的系统通知
pub fn send_simple_notification(title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 PowerShell 发送简单通知
    let script = format!(
        r#"
        Add-Type -AssemblyName System.Windows.Forms
        [System.Windows.Forms.NotifyIcon].assembly.location | Out-Null
        $notify = New-Object System.Windows.Forms.NotifyIcon
        $notify.Icon = [System.Drawing.SystemIcons]::Information
        $notify.BalloonTipTitle = '{}'
        $notify.BalloonTipText = '{}'
        $notify.BalloonTipIcon = 'Info'
        $notify.Visible = $true
        $notify.ShowBalloonTip(5000)
        Start-Sleep -Seconds 6
        $notify.Dispose()
        "#,
        title.replace("'", "''"),
        body.replace("'", "''")
    );

    Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-Command", &script])
        .output()
        .map_err(|e| Box::new(e))?;

    Ok(())
}

// 带 URL 的备用通知函数
pub fn send_notification_with_url_fallback(title: &str, body: &str, url: &str) {
    // 使用 PowerShell 创建可点击的通知气球
    let script = format!(
        r#"
        Add-Type -AssemblyName System.Windows.Forms
        Add-Type -AssemblyName System.Drawing
        
        $notify = New-Object System.Windows.Forms.NotifyIcon
        $notify.Icon = [System.Drawing.SystemIcons]::Information
        $notify.BalloonTipTitle = '{}'
        $notify.BalloonTipText = '{} (点击打开网站)'
        $notify.BalloonTipIcon = 'Info'
        $notify.Visible = $true

        # 添加点击事件
        $notify.add_BalloonTipClicked({{
            Start-Process '{}'
        }})

        $notify.ShowBalloonTip(10000)
        Start-Sleep -Seconds 12
        $notify.Dispose()
        "#,
        title.replace("'", "''"),
        body.replace("'", "''"),
        url
    );

    if let Err(e) = std::process::Command::new("powershell")
        .args(&["-WindowStyle", "Hidden", "-Command", &script])
        .output()
    {
        log::error!("备用通知发送失败: {:?}", e);
    }
}

// 公共函数：使用兼容 Windows 7 的通知方式
pub fn send_notification_with_fallback(title: &str, body: &str) {
    if let Err(e) = send_simple_notification(title, body) {
        log::error!("备用通知也发送失败: {:?}", e);
    }
}