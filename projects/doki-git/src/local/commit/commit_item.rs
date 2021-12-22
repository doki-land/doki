use super::*;



impl Debug for FileCommitItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("Commit");
        w.field("time", &self.time);
        w.field("commit_id", &self.id);
        w.field("lines", &self.lines);
        if let Some(s) = &self.name { w.field("name", s); }
        if let Some(s) = &self.email { w.field("email", s); }
        w.finish()
    }
}


impl From<BlameHunk<'_>> for FileCommitItem {
    fn from(blame: BlameHunk) -> Self {
        let sig = blame.orig_signature();
        Self {
            id: blame.orig_commit_id(),
            lines: blame.lines_in_hunk(),
            name: sig.name().map(|s| s.to_string()),
            email: sig.email().map(|s| s.to_string()),
            time: NaiveDateTime::from_timestamp(sig.when().seconds(), 0),
        }
    }
}