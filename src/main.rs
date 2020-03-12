use pathfinding::directed::dfs::dfs;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Page {
    Start,
    Success,
    Page(isize),
}

macro_rules! pagevec {
    ($($page:expr),*) => {
        vec![$(Page::Page($page)),*]
    }
}

impl Page {
    pub fn successors(&self) -> Vec<Self> {
        match self {
            Page::Start => pagevec![47, 177, 205],
            Page::Page(47) => pagevec![153, 206],
            Page::Page(153) => pagevec![17],
            Page::Page(17) => pagevec![35, 135],
            Page::Page(35) => pagevec![51, 68],
            Page::Page(51) => pagevec![68], // cycle
            Page::Page(68) => vec![], // cycle
            Page::Page(135) => pagevec![124, 183],
            Page::Page(124) => pagevec![116, 88],
            Page::Page(116) => pagevec![208],
            Page::Page(208) => vec![Page::Success],
            Page::Page(88) => pagevec![208],
            Page::Page(183) => pagevec![207, 208],
            Page::Page(207) => vec![],
            Page::Page(206) => vec![],
            Page::Page(177) => pagevec![130, 117],
            Page::Page(130) => pagevec![154, 39],
            Page::Page(154) => pagevec![17],
            Page::Page(39) => vec![],
            Page::Page(117) => pagevec![87, 50, 28],
            Page::Page(87) => pagevec![69, 40, 28],
            Page::Page(69) => vec![], // cycle
            Page::Page(28) => vec![], // cycle
            Page::Page(50) => vec![], // cycle
            Page::Page(40) => pagevec![178], // cycle
            Page::Page(178) => pagevec![208], // cycle
            Page::Page(205) => vec![],
            Page::Success => pagevec![],
            _ => unimplemented!("{:?}", self),
        }
    }
}

fn main() {
    println!("{:?}", dfs(Page::Start, Page::successors, |x| x == &Page::Success));
}

#[cfg(test)]
mod test {
    use super::*;

    fn validate_path(page: Page) {
        for page in page.successors() {
            validate_path(page);
        }
    }

    #[test]
    fn validate_graph() {
        validate_path(Page::Start);
    }
}
