//! Built-in sample documents and templates (original text, generated in code).

use vforge_word_doc::para::InlineObject;
use vforge_word_doc::props::{Align, Border, BorderStyle, Borders, CharProps, NumRef, Rgb, TabAlign, TabLeader, TabStop, TextColor};
use vforge_word_doc::{Block, Document, ListKind, Paragraph, PartKind, Table, para_block};

fn para(text: &str) -> Paragraph {
    Paragraph::with_text(text, CharProps::default())
}

fn styled(text: &str, style: &str) -> Paragraph {
    para(text).styled(style)
}

/// Append text with formatting to a paragraph.
fn add(p: &mut Paragraph, text: &str, props: CharProps) {
    let at = p.len();
    let _ = p.insert_text(at, text, &props);
}

fn bold() -> CharProps {
    CharProps { bold: Some(true), ..Default::default() }
}
fn italic() -> CharProps {
    CharProps { italic: Some(true), ..Default::default() }
}

/// The showcase document: a short studio handbook with headings, lists, a table, a quote,
/// a footer with page numbers.
pub fn sample_document() -> Document {
    let mut d = Document::new();
    d.styles.default_chr.lang = Some("vi-VN".into());
    d.core.title = "Sổ tay Studio Sáng tạo".into();
    d.core.creator = "VForge Word".into();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(styled("Sổ tay Studio Sáng tạo", "Title")));
    b.push(Block::Para(styled("Cách một cộng đồng cùng chia sẻ không gian, công cụ và thời gian", "Subtitle")));
    let mut intro = para("Mỗi tối thứ Năm, xưởng in thủ công trên con phố nhỏ lại rộn rã bước chân của những người say mê sáng tạo. ");
    add(&mut intro, "Họa sĩ vẽ tranh", bold());
    add(&mut intro, " chọn góc làm việc cạnh khung cửa sổ đón ánh sáng tự nhiên phía bắc, ", CharProps::default());
    add(&mut intro, "nhóm in ấn thủ công", italic());
    add(
        &mut intro,
        " tụ họp ở gian phòng phía sau, và mọi người luôn mang theo những món bánh thơm ngon để sẻ chia. Cuốn sổ tay này đúc kết những trải nghiệm thực tế trong việc vận hành một studio chung: các nếp sinh hoạt văn minh, hệ thống trang thiết bị dùng chung, và những thỏa thuận giản dị giúp những người xa lạ gắn kết thành một gia đình sáng tạo.",
        CharProps::default(),
    );
    b.push(Block::Para(intro));
    b.push(Block::Para(styled("Vì sao nên chia sẻ không gian studio?", "Heading1")));
    b.push(Block::Para(para("Tiết kiệm chi phí thuê mặt bằng là lợi ích dễ thấy nhất, nhưng chưa phải là điều giá trị nhất. Làm việc trong cùng một không gian giúp tác phẩm của bạn được đặt cạnh những góc nhìn nghệ thuật hoàn toàn mới mẻ. Một nghệ nhân làm gốm có thể say sưa giải thích về công thức men màu cho một họa sĩ truyện tranh; một nhiếp ảnh gia có thể mượn khung cửi dệt để tạo bối cảnh tĩnh vật độc đáo. Sự giao thoa và kết nối ý tưởng chính là nguồn cảm hứng vô tận.")));
    let mut quote = styled("Studio là nơi mọi ý tưởng còn dang dở đều được trân trọng và đón nhận.", "Quote");
    quote.props.space_before = Some(12.0);
    b.push(Block::Para(quote));
    b.push(Block::Para(styled("Quy ước chung", "Heading2")));
    let mut num = d.numbering.clone();
    let bullets = num.add_list(ListKind::Bullet);
    let steps = num.add_list(ListKind::Numbered);
    d.numbering = num;
    for (lead, rest) in [
        ("Gọn gàng sau khi làm việc. ", "Hãy giữ gìn bàn làm việc sạch đẹp như cách bạn mong muốn khi vừa bước vào xưởng."),
        ("Dán nhãn vật dụng cá nhân. ", "Nguyên vật liệu không ghi tên tại kệ chung sẽ được hiểu là đồ dùng tập thể."),
        ("Hỏi ý kiến trước khi mượn. ", "Ngay cả với đồ chung, một lời nhắn lịch sự sẽ giúp mọi người luôn nắm được tình hình."),
        ("Khung giờ tập trung tĩnh lặng ", "từ 9 giờ đến 11 giờ sáng các ngày trong tuần để làm việc chuyên sâu."),
    ] {
        let mut p = Paragraph::new().styled("ListParagraph");
        add(&mut p, lead, bold());
        add(&mut p, rest, CharProps::default());
        p.props.numbering = Some(NumRef { num: bullets, level: 0 });
        b.push(Block::Para(p));
    }
    b.push(Block::Para(styled("Trang thiết bị dùng chung", "Heading2")));
    b.push(Block::Para(para(
        "Bảng dưới đây thống kê các trang thiết bị thuộc sở hữu chung của xưởng và nhân sự chịu trách nhiệm vận hành, bảo dưỡng định kỳ.",
    )));
    let rows = [
        ["Thiết bị", "Vị trí", "Phụ trách", "Đăng ký sử dụng"],
        ["Máy in khắc kim loại", "Phòng kỹ thuật", "Minh Anh", "Bảng đăng ký trực tiếp"],
        ["Lò nung gốm điện", "Khu xưởng ngoài", "Tuấn Phong", "Thông báo trước 48 giờ"],
        ["Máy in ảnh khổ lớn", "Văn phòng trung tâm", "Thu Hà", "Lịch trực tuyến"],
        ["Bàn soi sáng mỹ thuật", "Dãy cửa sổ bắc", "Tất cả thành viên", "Tự do sử dụng"],
    ];
    let mut t = Table::new(rows.len(), 4, 468.0);
    t.props.style = Some("GridTable4AccentBlue".into());
    for (r, row) in rows.iter().enumerate() {
        for (c, txt) in row.iter().enumerate() {
            if let Some(cell) = t.rows.get_mut(r).and_then(|x| x.cells.get_mut(c)) {
                cell.blocks = vec![para_block(para(txt))];
            }
        }
    }
    if let Some(r) = t.rows.first_mut() {
        r.props.header = true;
    }
    b.push(Block::Table(t));
    b.push(Block::Para(styled("Quy trình mở cửa studio", "Heading2")));
    for s in [
        "Bật cầu dao điện chính cạnh cửa ra vào.",
        "Mở các cửa kính thông gió đón ánh sáng tự nhiên (trừ khi trời mưa).",
        "Bật ấm đun nước pha trà và cà phê. Đây là bước khởi động ngày mới tuyệt vời!",
        "Kiểm tra danh sách lịch đặt trước cho máy in và lò nung.",
    ] {
        let mut p = para(s).styled("ListParagraph");
        p.props.numbering = Some(NumRef { num: steps, level: 0 });
        b.push(Block::Para(p));
    }
    b.push(Block::Para(styled("Thành viên & Gia nhập", "Heading1")));
    let mut m = para("Hình thức đóng phí thành viên là ");
    add(&mut m, "tùy theo khả năng", CharProps { bold: Some(true), color: Some(TextColor::Rgb(Rgb(0x15, 0x60, 0x82))), ..Default::default() });
    add(
        &mut m,
        ", được định kỳ xem xét theo từng mùa. Thành viên mới sẽ được đồng hành cùng một thành viên kỳ cựu trong hai buổi đầu tiên, và mỗi người sẽ tham gia một ca trực vệ sinh mỗi tháng. Dù bạn chưa từng tạo ra sản phẩm nghệ thuật nào trước đây, bạn chính là người mà chúng tôi luôn hân hoan chào đón.",
        CharProps::default(),
    );
    b.push(Block::Para(m));
    let mut h = para("Mọi câu hỏi và đóng góp ý kiến vui lòng gửi về bàn tiếp tân, hoặc ghi trên bảng thông báo cạnh khu vực pha chế. ");
    add(
        &mut h,
        "Ghé thăm trang cộng đồng",
        CharProps { style: Some("Hyperlink".into()), link: Some("https://github.com/huytran1120/vforge-word".into()), ..Default::default() },
    );
    add(&mut h, " để theo dõi các workshop và sự kiện sắp diễn ra.", CharProps::default());
    b.push(Block::Para(h));
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    // Footer: centred page number.
    let mut f = Paragraph::new().styled("Footer");
    f.props.align = Some(Align::Center);
    let _ = f.insert_object(
        0,
        InlineObject::Field { instr: "PAGE".into(), result: "1".into(), locked: false, dirty: false, code: false, form: String::new() },
        &CharProps::default(),
    );
    let fid = d.add_part(PartKind::Footer, vec![para_block(f)]);
    d.last_section.footers.default = Some(fid);
    let mut hd = styled("Sổ tay Studio Sáng tạo\t\tBản Mùa Xuân", "Header");
    hd.props.borders = Some(Borders {
        bottom: Some(Border { style: BorderStyle::Single, width: 0.5, color: Some(Rgb(0xBF, 0xBF, 0xBF)), space: 4.0 }),
        ..Default::default()
    });
    let hid = d.add_part(PartKind::Header, vec![para_block(hd)]);
    d.last_section.headers.default = Some(hid);
    d
}

/// A block-style letter.
pub fn letter() -> Document {
    let mut d = Document::new();
    let lines: Vec<Paragraph> = vec![
        styled("Nguyễn Văn An", "Title"),
        para("123 Đường Lê Lợi, Phường Bến Nghé, Quận 1, TP. Hồ Chí Minh · 090 123 4567 · an.nguyen@example.com"),
        para(""),
        para(&format!("Ngày {}", crate::cmd::insert::format_date("dd/MM/yyyy"))),
        para(""),
        para("Kính gửi: Ban Giám đốc"),
        para("Công ty TNHH Sáng tạo & Công nghệ"),
        para("456 Đường Nguyễn Huệ, Quận 1, TP. Hồ Chí Minh"),
        para(""),
        para("Kính gửi Quý đối tác,"),
        para(
            "Tôi viết thư này nhằm đề xuất kế hoạch hợp tác triển khai dự án số hóa tài liệu và nâng cao năng suất xử lý văn bản cho doanh nghiệp. Kính mời Quý công ty xem xét bản đề án tóm tắt đính kèm và cùng thảo luận chi tiết trong buổi gặp gỡ vào tuần tới.",
        ),
        para("Xin trân trọng cảm ơn sự quan tâm và thời gian quý báu của Quý công ty."),
        para("Kính thư,"),
        para(""),
        para("Nguyễn Văn An"),
    ];
    d.body = lines.into_iter().map(para_block).collect();
    d
}

/// A one-page résumé.
pub fn resume() -> Document {
    let mut d = Document::new();
    let mut b: Vec<Block> = vec![
        Block::Para(styled("Trần Minh Quân", "Title")),
        Block::Para(para("Chuyên viên Thiết kế Đồ họa & Sáng tạo · minhquan@example.com · portfolio.example.com")),
    ];
    for (head, items) in [
        (
            "Kinh nghiệm làm việc",
            vec![
                ("Trưởng nhóm Thiết kế, Công ty Truyền thông & Xuất bản", "2021 – Hiện tại"),
                ("Chuyên viên Thiết kế Đồ họa & Minh họa Tự do", "2016 – 2021"),
            ],
        ),
        ("Học vấn", vec![("Cử nhân Thiết kế Đồ họa, Trường Đại học Mỹ thuật", "2016")]),
        (
            "Kỹ năng chuyên môn",
            vec![
                ("Màu nước, sơn dầu, minh họa kỹ thuật số, dàn trang sách, nghệ thuật chữ (typography)", ""),
                ("Phần mềm: VForge Word, Photoshop, Illustrator, InDesign, Figma", ""),
            ],
        ),
    ] {
        b.push(Block::Para(styled(head, "Heading1")));
        for (what, when) in items {
            let mut p = para(what);
            p.props.tabs = Some(vec![TabStop { pos: 468.0, align: TabAlign::Right, leader: TabLeader::None }]);
            if !when.is_empty() {
                add(&mut p, &format!("\t{when}"), italic());
            }
            b.push(Block::Para(p));
        }
    }
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    d
}

/// A report with a cover heading and sections.
pub fn report() -> Document {
    let mut d = Document::new();
    let b: Vec<Block> = vec![
        Block::Para(styled("Báo cáo Tổng kết Dự án", "Title")),
        Block::Para(styled("Đánh giá kết quả hoạt động và định hướng kế hoạch tiếp theo", "Subtitle")),
        Block::Para(styled("Tóm tắt điều hành", "Heading1")),
        Block::Para(para(
            "Bản báo cáo tổng hợp các kết quả trọng tâm đã đạt được trong giai đoạn vừa qua, đồng thời phân tích các chỉ số đánh giá hiệu quả then chốt và đưa ra đề xuất cho giai đoạn tiếp theo.",
        )),
        Block::Para(styled("Bối cảnh & Mục tiêu", "Heading1")),
        Block::Para(para(
            "Mô tả bối cảnh triển khai, các yêu cầu thực tiễn và những câu hỏi then chốt mà dự án cần giải quyết để đáp ứng mục tiêu đề ra.",
        )),
        Block::Para(styled("Kết quả & Phát hiện chủ yếu", "Heading1")),
        Block::Para(styled("Hiệu quả nâng cao năng suất", "Heading2")),
        Block::Para(para(
            "Trình bày các số liệu thực chứng, biểu đồ phân tích và phân tích chuyên sâu về sự tiến bộ vượt bậc trong quy trình làm việc.",
        )),
        Block::Para(styled("Đề xuất & Kế hoạch hành động", "Heading1")),
        Block::Para(para("Liệt kê danh sách các bước hành động cụ thể, thời hạn hoàn thành và phân công trách nhiệm cho các bộ phận liên quan.")),
    ];
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_are_valid() {
        assert_eq!(sample_document().word_count(), 646);
        for d in [sample_document(), letter(), resume(), report()] {
            assert!(d.word_count() > 5);
            let l = vforge_word_layout::layout(&d, &mut vforge_word_layout::LayoutCache::new(), &Default::default());
            assert!(!l.pages.is_empty());
        }
    }
}
