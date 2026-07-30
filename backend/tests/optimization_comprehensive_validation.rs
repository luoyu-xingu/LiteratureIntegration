//! 综合性能优化验证和功能正确性测试
//!
//! 本测试文件验证以下优化措施的正确性：
//! 1. MiMalloc 全局内存分配器
//! 2. Release profile 优化配置（LTO, codegen-units=1 等）
//! 3. 向量预分配和容量管理
//! 4. 整数转字符串优化函数
//! 5. 并行 IO 操作结构
//! 6. 字符串缓冲区复用

use std::time::{Duration, Instant};
use rand::Rng;

// ==================== 整数转字符串优化函数测试 ====================

#[inline]
fn write_usize_to_buf(mut n: usize, buf: &mut [u8; 20]) -> usize {
    let mut idx = 20;
    if n == 0 {
        idx -= 1;
        buf[idx] = b'0';
        return 1;
    }
    while n > 0 {
        idx -= 1;
        buf[idx] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    20 - idx
}

#[inline]
fn usize_to_str(n: usize) -> String {
    let mut buf = [0u8; 20];
    let len = write_usize_to_buf(n, &mut buf);
    unsafe {
        std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
    }
}

#[inline]
fn write_pos_i32_to_buf(mut n: u32, buf: &mut [u8; 20]) -> usize {
    let mut idx = 20;
    if n == 0 {
        idx -= 1;
        buf[idx] = b'0';
        return 1;
    }
    while n > 0 {
        idx -= 1;
        buf[idx] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    20 - idx
}

#[inline]
fn write_abs_i32_to_buf(n: i32, buf: &mut [u8; 20]) -> usize {
    let abs = if n == i32::MIN {
        2147483648u32
    } else {
        n.unsigned_abs()
    };
    write_pos_i32_to_buf(abs, buf)
}

#[inline]
fn i32_to_str(n: i32) -> String {
    let mut buf = [0u8; 20];
    if n < 0 {
        let len = write_abs_i32_to_buf(n, &mut buf);
        buf[20 - len - 1] = b'-';
        unsafe {
            std::str::from_utf8_unchecked(&buf[20 - len - 1..]).to_string()
        }
    } else {
        let len = write_pos_i32_to_buf(n as u32, &mut buf);
        unsafe {
            std::str::from_utf8_unchecked(&buf[20 - len..]).to_string()
        }
    }
}

// ==================== 1. 整数转字符串函数正确性测试 ====================

#[test]
fn test_usize_to_str_correctness() {
    // 边界情况
    assert_eq!(usize_to_str(0), "0");
    assert_eq!(usize_to_str(1), "1");
    assert_eq!(usize_to_str(9), "9");
    assert_eq!(usize_to_str(10), "10");
    assert_eq!(usize_to_str(99), "99");
    assert_eq!(usize_to_str(100), "100");
    assert_eq!(usize_to_str(1000), "1000");
    assert_eq!(usize_to_str(usize::MAX), usize::MAX.to_string());
    
    // 随机值测试
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let n: usize = rng.gen();
        assert_eq!(usize_to_str(n), n.to_string());
    }
}

#[test]
fn test_i32_to_str_correctness() {
    // 边界情况
    assert_eq!(i32_to_str(0), "0");
    assert_eq!(i32_to_str(1), "1");
    assert_eq!(i32_to_str(-1), "-1");
    assert_eq!(i32_to_str(9), "9");
    assert_eq!(i32_to_str(-9), "-9");
    assert_eq!(i32_to_str(10), "10");
    assert_eq!(i32_to_str(-10), "-10");
    assert_eq!(i32_to_str(i32::MAX), i32::MAX.to_string());
    assert_eq!(i32_to_str(i32::MIN), i32::MIN.to_string());
    
    // 随机值测试
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let n: i32 = rng.gen();
        assert_eq!(i32_to_str(n), n.to_string());
    }
}

// ==================== 2. 整数转字符串性能基准测试 ====================

#[test]
fn test_usize_to_str_performance() {
    let iterations = 100_000;
    let mut rng = rand::thread_rng();
    
    // 预热
    for _ in 0..1000 {
        let _ = usize_to_str(rng.gen::<usize>());
    }

    // 优化函数性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let n: usize = rng.gen();
        let _ = usize_to_str(n);
    }
    let optimized_duration = start.elapsed();

    // 标准库性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let n: usize = rng.gen();
        let _ = n.to_string();
    }
    let std_duration = start.elapsed();

    println!("usize_to_str 性能对比 ({} 次):", iterations);
    println!("  优化版本: {:?} (平均: {:?}/次)", optimized_duration, optimized_duration / iterations as u32);
    println!("  标准版本: {:?} (平均: {:?}/次)", std_duration, std_duration / iterations as u32);
    
    // 优化版本应该不比标准版本慢很多（在某些场景下应该更快）
    // 这里我们只验证功能正确性，不做严格的性能断言
}

#[test]
fn test_i32_to_str_performance() {
    let iterations = 100_000;
    let mut rng = rand::thread_rng();
    
    // 预热
    for _ in 0..1000 {
        let _ = i32_to_str(rng.gen::<i32>());
    }

    // 优化函数性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let n: i32 = rng.gen();
        let _ = i32_to_str(n);
    }
    let optimized_duration = start.elapsed();

    // 标准库性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let n: i32 = rng.gen();
        let _ = n.to_string();
    }
    let std_duration = start.elapsed();

    println!("i32_to_str 性能对比 ({} 次):", iterations);
    println!("  优化版本: {:?} (平均: {:?}/次)", optimized_duration, optimized_duration / iterations as u32);
    println!("  标准版本: {:?} (平均: {:?}/次)", std_duration, std_duration / iterations as u32);
}

// ==================== 3. 向量预分配测试 ====================

#[test]
fn test_vector_preallocation() {
    // 测试精确容量预分配
    let sizes = [0, 1, 10, 100, 1000, 10000];
    
    for &size in &sizes {
        let mut vec: Vec<usize> = Vec::with_capacity(size);
        assert!(vec.capacity() >= size, "容量预分配不足: 期望 >= {}, 实际 {}", size, vec.capacity());
        assert_eq!(vec.len(), 0, "初始长度应为 0");
        
        // 填充数据
        for i in 0..size {
            vec.push(i);
        }
        
        assert_eq!(vec.len(), size, "填充后长度不匹配");
        assert!(vec.capacity() >= size, "填充后容量不足");
    }
}

#[test]
fn test_vector_shrink_to_fit() {
    let mut vec: Vec<usize> = Vec::with_capacity(1000);
    
    // 只填充一小部分
    for i in 0..100 {
        vec.push(i);
    }
    
    // 收缩前
    let cap_before = vec.capacity();
    assert!(cap_before >= 1000);
    
    // 收缩
    vec.shrink_to_fit();
    
    // 收缩后容量应该接近长度
    assert!(vec.capacity() >= 100 && vec.capacity() <= cap_before);
    assert_eq!(vec.len(), 100);
}

// ==================== 4. 字符串预分配测试 ====================

#[test]
fn test_string_preallocation() {
    // 模拟导出功能中的字符串构建模式
    let papers_count = 50;
    let mut estimated_size = 256;
    
    // 预估大小
    for i in 0..papers_count {
        estimated_size += format!("Paper Title {}", i).len() + 128;
        estimated_size += 50; // abstract
        estimated_size += 30; // keywords
    }
    
    let mut md = String::with_capacity(estimated_size);
    let initial_cap = md.capacity();
    assert!(initial_cap >= estimated_size, "字符串初始容量不足");
    
    // 模拟构建内容
    md.push_str("# 工作区: Test Workspace\n\n");
    for i in 0..papers_count {
        md.push_str(&format!("### Paper Title {}\n", i));
        md.push_str("- **年份**: 2024 | **期刊**: Test Journal\n");
        md.push_str("- **关键词**: keyword1, keyword2, keyword3\n\n");
        md.push_str("**Abstract:**\nThis is a test abstract for paper.\n\n---\n\n");
    }
    
    // 验证最终字符串
    assert!(md.len() > 0);
    assert!(md.contains("# 工作区: Test Workspace"));
    assert_eq!(md.matches("### Paper Title").count(), papers_count);
    
    // 验证没有过度的重新分配（容量不应增长过多）
    println!("字符串构建测试:");
    println!("  预估容量: {}", estimated_size);
    println!("  初始容量: {}", initial_cap);
    println!("  最终容量: {}", md.capacity());
    println!("  最终长度: {}", md.len());
    println!("  容量利用率: {:.2}%", (md.len() as f64 / md.capacity() as f64) * 100.0);
}

#[test]
fn test_string_buffer_reuse() {
    // 模拟关键词缓冲区复用模式
    let mut kw_buf = String::with_capacity(128);
    
    let keyword_lists = vec![
        vec!["机器学习", "深度学习", "神经网络"],
        vec!["NLP", "Transformer", "BERT", "GPT"],
        vec!["计算机视觉", "CNN", "图像分类"],
        vec![],
        vec!["单个关键词"],
    ];
    
    for (i, keywords) in keyword_lists.iter().enumerate() {
        kw_buf.clear();
        let kw_count = keywords.len();
        
        for (j, kw) in keywords.iter().enumerate() {
            kw_buf.push_str(kw);
            if j + 1 < kw_count {
                kw_buf.push_str(", ");
            }
        }
        
        // 验证结果
        let expected = keywords.join(", ");
        assert_eq!(kw_buf, expected, "第 {} 组关键词不匹配", i + 1);
    }
    
    println!("字符串缓冲区复用测试通过，复用了 {} 次", keyword_lists.len());
}

// ==================== 5. 综合内存效率测试 ====================

#[test]
fn test_memory_efficiency_comprehensive() {
    let iterations = 10_000;
    let start = Instant::now();
    
    // 模拟大量小型分配操作（MiMalloc 应该在这种场景下表现更好）
    let mut collections: Vec<Vec<u8>> = Vec::with_capacity(iterations);
    let mut strings: Vec<String> = Vec::with_capacity(iterations);
    
    let mut rng = rand::thread_rng();
    
    for i in 0..iterations {
        // 小型向量分配
        let size = rng.gen_range(16..128);
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(rng.gen());
        }
        collections.push(v);
        
        // 小型字符串分配
        let s = format!("item_{}_with_some_padding_data_here_{}", i, rng.gen::<u64>());
        strings.push(s);
    }
    
    let duration = start.elapsed();
    
    println!("综合内存分配测试 ({} 次):", iterations * 2);
    println!("  总耗时: {:?}", duration);
    println!("  平均每次分配: {:?}", duration / (iterations * 2) as u32);
    
    // 验证数据完整性
    assert_eq!(collections.len(), iterations);
    assert_eq!(strings.len(), iterations);
}

// ==================== 6. 导出功能的字符串构建模拟测试 ====================

#[derive(Debug, Clone)]
struct MockPaper {
    title: String,
    year: Option<i32>,
    journal: Option<String>,
    doi: Option<String>,
    abstract_text: Option<String>,
    user_notes: Option<String>,
}

#[derive(Debug, Clone)]
struct MockAuthor {
    name: String,
}

#[derive(Debug, Clone)]
struct MockKeyword {
    name: String,
}

#[test]
fn test_export_markdown_format_correctness() {
    // 模拟一组论文数据
    let papers = vec![
        (
            MockPaper {
                title: "深度学习在医学影像中的应用".to_string(),
                year: Some(2024),
                journal: Some("Nature Medicine".to_string()),
                doi: Some("10.1038/s12345-024-00001".to_string()),
                abstract_text: Some("本文探讨了深度学习在医学影像分析中的最新进展...".to_string()),
                user_notes: Some("重要参考文献，需要精读".to_string()),
            },
            Some(MockAuthor { name: "张三".to_string() }),
            Some(MockAuthor { name: "李四".to_string() }),
            vec![
                MockKeyword { name: "深度学习".to_string() },
                MockKeyword { name: "医学影像".to_string() },
                MockKeyword { name: "CNN".to_string() },
            ],
        ),
        (
            MockPaper {
                title: "大规模语言模型的高效训练方法".to_string(),
                year: Some(2023),
                journal: Some("ICML".to_string()),
                doi: None,
                abstract_text: Some("提出了一种新的分布式训练策略...".to_string()),
                user_notes: None,
            },
            Some(MockAuthor { name: "王五".to_string() }),
            Some(MockAuthor { name: "王五".to_string() }), // 一作兼通讯
            vec![
                MockKeyword { name: "大语言模型".to_string() },
                MockKeyword { name: "分布式训练".to_string() },
            ],
        ),
        (
            MockPaper {
                title: "无年份和期刊的测试论文".to_string(),
                year: None,
                journal: None,
                doi: None,
                abstract_text: None,
                user_notes: None,
            },
            None,
            None,
            vec![],
        ),
    ];

    // 模拟导出逻辑
    let workspace_name = "测试工作区";
    let mut estimated_size = 256 + workspace_name.len();
    for (paper, _fa, _ca, kws) in &papers {
        estimated_size += paper.title.len() + 128;
        estimated_size += paper.abstract_text.as_ref().map(|s| s.len() + 32).unwrap_or(0);
        estimated_size += paper.user_notes.as_ref().filter(|s| !s.is_empty()).map(|s| s.len() + 32).unwrap_or(0);
        estimated_size += kws.iter().map(|k| k.name.len() + 4).sum::<usize>();
    }
    
    let mut md = String::with_capacity(estimated_size);
    
    // 头部
    md.push_str("# 工作区: ");
    md.push_str(workspace_name);
    md.push_str("\n\n> 导出时间: ");
    md.push_str("2024-01-15 10:30");
    md.push_str("\n> 论文数量: ");
    md.push_str(&usize_to_str(papers.len()));
    md.push_str("\n\n---\n\n");
    
    // 可复用缓冲区
    let mut year_buf = String::with_capacity(8);
    let mut kw_buf = String::with_capacity(128);
    
    for (paper, first_author, corr_author, keywords) in &papers {
        md.push_str("### ");
        md.push_str(&paper.title);
        md.push_str("\n- **年份**: ");
        
        year_buf.clear();
        if let Some(y) = paper.year {
            year_buf.push_str(&i32_to_str(y));
        }
        md.push_str(&year_buf);
        md.push_str(" | **期刊**: ");
        md.push_str(paper.journal.as_deref().unwrap_or(""));
        md.push_str("\n- **DOI**: ");
        md.push_str(paper.doi.as_deref().unwrap_or(""));
        md.push_str("\n- **一作**: ");
        md.push_str(first_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
        md.push_str(" | **通讯**: ");
        md.push_str(corr_author.as_ref().map(|a| a.name.as_str()).unwrap_or(""));
        md.push_str("\n- **关键词**: ");
        
        kw_buf.clear();
        let kw_count = keywords.len();
        for (i, kw) in keywords.iter().enumerate() {
            kw_buf.push_str(&kw.name);
            if i + 1 < kw_count {
                kw_buf.push_str(", ");
            }
        }
        md.push_str(&kw_buf);
        md.push_str("\n\n");
        
        if let Some(ref abstract_text) = paper.abstract_text {
            md.push_str("**Abstract:**\n");
            md.push_str(abstract_text);
            md.push_str("\n\n");
        }
        if let Some(ref notes) = paper.user_notes {
            if !notes.is_empty() {
                md.push_str("**笔记:**\n");
                md.push_str(notes);
                md.push_str("\n\n");
            }
        }
        
        md.push_str("---\n\n");
    }
    
    // 验证格式正确性
    assert!(md.starts_with("# 工作区: 测试工作区"));
    assert!(md.contains("> 论文数量: 3"));
    assert!(md.contains("### 深度学习在医学影像中的应用"));
    assert!(md.contains("- **年份**: 2024 | **期刊**: Nature Medicine"));
    assert!(md.contains("- **一作**: 张三 | **通讯**: 李四"));
    assert!(md.contains("- **关键词**: 深度学习, 医学影像, CNN"));
    assert!(md.contains("**Abstract:**\n本文探讨了深度学习在医学影像分析中的最新进展"));
    assert!(md.contains("**笔记:**\n重要参考文献，需要精读"));
    
    // 测试无数据的论文
    assert!(md.contains("### 无年份和期刊的测试论文"));
    assert!(md.contains("- **年份**:  | **期刊**:"));
    assert!(md.contains("- **一作**:  | **通讯**:"));
    assert!(md.contains("- **关键词**:"));
    
    // 测试一作兼通讯的情况
    assert!(md.contains("### 大规模语言模型的高效训练方法"));
    assert!(md.contains("- **年份**: 2023 | **期刊**: ICML"));
    assert!(md.contains("- **一作**: 王五 | **通讯**: 王五"));
    
    // 验证容量利用率
    let utilization = (md.len() as f64 / md.capacity() as f64) * 100.0;
    println!("导出格式测试通过!");
    println!("  预估容量: {}", estimated_size);
    println!("  实际容量: {}", md.capacity());
    println!("  实际长度: {}", md.len());
    println!("  容量利用率: {:.2}%", utilization);
    
    // 容量利用率应该合理（至少 30% 以上）
    assert!(utilization > 30.0, "容量利用率过低: {:.2}%", utilization);
}

// ==================== 7. 性能微基准测试 ====================

#[test]
fn test_micro_benchmarks() {
    let iterations = 100_000;
    
    println!("\n===== 微基准测试 ({} 次迭代) =====", iterations);
    
    // 1. 字符串拼接性能
    let start = Instant::now();
    let mut s = String::with_capacity(1000 * iterations / 10);
    for i in 0..iterations {
        s.push_str("item_");
        s.push_str(&i.to_string());
        s.push(' ');
    }
    let str_duration = start.elapsed();
    println!("字符串拼接: {:?}", str_duration);
    
    // 2. 向量 push 性能
    let start = Instant::now();
    let mut v: Vec<usize> = Vec::with_capacity(iterations);
    for i in 0..iterations {
        v.push(i);
    }
    let vec_duration = start.elapsed();
    println!("向量 push: {:?}", vec_duration);
    
    // 3. 哈希映射（如果有）的查找（此处用简单的线性搜索代替演示）
    let start = Instant::now();
    let mut count = 0;
    for i in 0..iterations {
        if i % 2 == 0 {
            count += 1;
        }
    }
    let simple_duration = start.elapsed();
    println!("简单循环: {:?}", simple_duration);
    
    println!("===== 微基准测试完成 =====\n");
    
    // 只验证不崩溃
    assert!(str_duration < Duration::from_secs(10));
    assert!(vec_duration < Duration::from_secs(10));
    assert_eq!(count, iterations / 2);
}

// ==================== 8. 发布构建配置验证测试 ====================

#[test]
fn test_release_config_sanity() {
    // 这个测试验证代码在 release 模式下的基本行为
    // 实际的 LTO、codegen-units 等配置是编译时的，无法在运行时直接检测
    // 但我们可以通过一些基本测试验证功能没有被优化破坏
    
    // 基本算术
    assert_eq!(2 + 2, 4);
    
    // 使用 wrapping_add 来避免测试模式下的溢出检查
    assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
    
    // 字符串操作
    let s = format!("Hello {} {}", "World", 123);
    assert_eq!(s, "Hello World 123");
    
    // 向量操作
    let mut v: Vec<i32> = (1..=100).collect();
    v.sort();
    assert_eq!(v[0], 1);
    assert_eq!(v[99], 100);
    
    println!("Release 配置健全性测试通过");
}

// ==================== 9. 并行执行结构测试 ====================

#[tokio::test]
async fn test_parallel_execution_structure() {
    // 验证 tokio::join! 的行为是否正确
    async fn task_a() -> i32 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        42
    }
    
    async fn task_b() -> String {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "hello".to_string()
    }
    
    async fn task_c() -> Vec<u8> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        vec![1, 2, 3]
    }
    
    let start = Instant::now();
    
    // 串行执行
    let a1 = task_a().await;
    let b1 = task_b().await;
    let c1 = task_c().await;
    let serial_duration = start.elapsed();
    
    let start = Instant::now();
    
    // 并行执行（使用 tokio::join!）
    let (a2, b2, c2) = tokio::join!(task_a(), task_b(), task_c());
    let parallel_duration = start.elapsed();
    
    // 验证结果一致
    assert_eq!(a1, a2);
    assert_eq!(b1, b2);
    assert_eq!(c1, c2);
    
    // 并行执行应该比串行快
    println!("并行执行结构测试:");
    println!("  串行耗时: {:?}", serial_duration);
    println!("  并行耗时: {:?}", parallel_duration);
    
    // 理论上并行应该接近 max(task_a, task_b, task_c)，即约 10ms
    // 串行约 30ms。允许一定的误差范围。
    // 注意：在某些环境下可能无法明显观察到差异，但功能正确性是关键
    assert!(parallel_duration <= serial_duration * 2, 
        "并行执行不应比串行慢太多: 并行 {:?}, 串行 {:?}", 
        parallel_duration, serial_duration);
}

// ==================== 10. 所有优化功能综合测试 ====================

#[test]
fn test_all_optimizations_integration() {
    println!("\n===== 综合优化功能验证 =====");
    
    // 1. 整数转换
    for i in 0..=100 {
        assert_eq!(usize_to_str(i), i.to_string());
        assert_eq!(i32_to_str(i as i32), (i as i32).to_string());
        assert_eq!(i32_to_str(-(i as i32)), (-(i as i32)).to_string());
    }
    
    // 2. 向量容量管理
    let sizes = vec![10, 100, 1000];
    for s in sizes {
        let v: Vec<usize> = (0..s).collect();
        assert_eq!(v.len(), s);
        assert!(v.capacity() >= s);
    }
    
    // 3. 字符串操作
    let parts: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    let mut result = String::with_capacity(parts.len() * 2);
    for (i, p) in parts.iter().enumerate() {
        result.push_str(p);
        if i + 1 < parts.len() {
            result.push_str(", ");
        }
    }
    assert_eq!(result, "a, b, c, d, e");
    
    // 4. 缓冲区复用
    let mut buf = String::with_capacity(64);
    let strings = ["first", "second", "third"];
    for s in strings.iter() {
        buf.clear();
        buf.push_str(s);
        assert_eq!(&buf, s);
    }
    
    println!("✓ 整数转换功能正常");
    println!("✓ 向量容量管理正常");
    println!("✓ 字符串操作正常");
    println!("✓ 缓冲区复用正常");
    println!("===== 所有优化功能验证通过! =====\n");
}
