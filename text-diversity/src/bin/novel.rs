use text_diversity::{pairwise_jaccard_diversity, distinct_n, self_bleu_diversity};

fn main() {
    let haiku = vec![
        "午夜 时分 她 在 楼梯口 发现 了 那 具 尸体 死者 是 邻居 王先生 但 奇怪 的 是 他 的 手里 紧握 着 一把 生锈 的 钥匙 警察 还没 到 她 却 发现 了 一个 更 恐怖 的 事实 那把 钥匙 正是 她 丢失 三年 的 那把",
        "半夜 李明 被 一通 无号码 来电 惊醒 话筒 里 传来 熟悉 却 陌生 的 声音 我 就 站 在 你家 楼下 你 看得见 我 吗 他 冲 到 窗边 黑暗 中 什么 都 看不清 电话 再次 响起 声音 更 近 了 抬起头",
        "夜幕 降临 林 深处 传来 一声 尖叫 警察 赶到 时 发现 受害者 已 无 生命 迹象 现场 没有 打斗 痕迹 没有 凶器 甚至 没有 血迹 只有 一只 沾满 泥土 的 女式 高跟鞋 指向 一个 令人 窒息 的 真相",
        "午夜 时分 林晓雨 在 废弃 工厂 里 发现 了 一具 尸体 但 最 诡异 的 是 死者 的 脸 和 她 的 镜子 里 的 自己 一模一样 她 颤抖 地 掏出 手机 却 发现 所有 通讯 记录 都 被 删除 了 身后 传来 缓慢 而 坚定 的 脚步声",
        "午夜 的 车站 空无一人 她 发现 口袋 里 多 了 张 陌生 的 纸条 字迹 熟悉 得 吓人 正是 三年 前 失踪 的 丈夫 的 笔迹 纸条 上 只有 五个 字 别 相信 任何人 门外 脚步声 渐近",
    ];

    let sonnet = vec![
        "午夜 三点 林雨桐 接到 一通 陌生 来电 电话 那头 传来 自己 的 声音 别 回头 镜子 里 的 人 不是 你 她 僵住 了 因为 此刻 她 正 独自 站 在 卫生间 的 镜子 前",
        "午夜 侦探 李墨 推开 死者 书房 的 门 墙上 时钟 停 在 11:47 地上 散落 着 撕碎 的 信件 他 弯腰 捡起 一片 纸屑 瞳孔 骤然 收缩 那 是 他 自己 的 笔迹",
        "午夜 钟声 敲响 时 林晓 发现 书房 的 窗户 从 内侧 反锁 了 可 他 独自 住 在 十二楼 刚才 明明 听见 有人 从 窗外 离开 的 声音",
        "午夜 雨打 窗棂 李墨 推开 书房 门 父亲 伏案 而 死 手中 攥 着 半张 发黄 的 照片 照片 上 另一半 人脸 正是 三年 前 失踪 的 母亲 桌上 留言 只有 一个 字 逃",
        "午夜 三点 林慧 推开 书房 门 父亲 伏 在 桌上 手中 紧握 一张 泛黄 的 照片 她 凑近 一看 照片 里 的 女人 竟 与 自己 一模一样 拍摄 日期 却是 1989年",
    ];

    let opus = vec![
        "深夜 老宅 的 钟 敲响 十二 下 林薇 推开 尘封 的 书房 手电筒 的 光束 扫过 墙面 那幅 全家福 里 父亲 的 脸 被人 用 刀片 刮去 了 而 父亲 已经 失踪 整整 三年",
        "深夜 老宅 的 钟声 敲响 十二 下 林墨 推开 书房 的 门 发现 父亲 倒 在 血泊 中 手里 紧握 着 一张 泛黄 的 照片 照片 上 的 人 竟是 三十年 前 失踪 的 母亲",
        "深夜 李探长 盯 着 桌上 的 照片 死者 微笑 着 手指 却 指向 窗外 三年 前 同样 的 姿势 同样 的 地点 但 那个 人 明明 是 他 亲手 埋葬 的",
        "深夜 老宅 的 钟 敲 了 十三 下 李探长 低头 看向 地上 的 尸体 死者 的 手指 指向 墙上 一幅 褪色 的 全家福 照片 里 有 七个 人 但 他 清楚 地 记得 这家 只有 六口 人",
        "深夜 老宅 的 钟声 敲响 十二 下 李探长 推开 那扇 尘封 多年 的 门 手电筒 的 光束 扫过 墙壁 那里 挂 着 一幅 全家福 但 每个 人 的 脸 都 被 利器 划去 只剩下 诡异 的 空洞",
    ];

    println!("=== 悬疑小说开头 多样性对比 ===\n");
    
    print_metrics("Haiku", &haiku);
    print_metrics("Sonnet", &sonnet);
    print_metrics("Opus", &opus);

    println!("--- 原文展示 ---\n");
    println!("[Haiku]");
    for (i, r) in haiku.iter().enumerate() {
        println!("{}. {}", i+1, r.replace(" ", ""));
    }
    println!("\n[Sonnet]");
    for (i, r) in sonnet.iter().enumerate() {
        println!("{}. {}", i+1, r.replace(" ", ""));
    }
    println!("\n[Opus]");
    for (i, r) in opus.iter().enumerate() {
        println!("{}. {}", i+1, r.replace(" ", ""));
    }
}

fn print_metrics(model: &str, responses: &[&str]) {
    println!("[{}]", model);
    println!("  Pairwise-Jaccard: {:.3}", pairwise_jaccard_diversity(responses));
    println!("  Distinct-1:       {:.3}", distinct_n(responses, 1));
    println!("  Distinct-2:       {:.3}", distinct_n(responses, 2));
    println!("  Self-BLEU:        {:.3}", 1.0 - self_bleu_diversity(responses, 4));
    println!();
}
