use {
    std::{
        collections::BTreeMap,
        path::PathBuf,
    },
    chrono::{
        prelude::*,
        Weekday::*,
    },
    if_chain::if_chain,
};

#[derive(clap::Parser)]
#[clap(version)]
struct Args {
    repo_path: Option<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] GitCommit(#[from] gix::object::commit::Error),
    #[error(transparent)] GitDiscover(#[from] gix::discover::Error),
    #[error(transparent)] GitHeadCommit(#[from] gix::reference::head_commit::Error),
    #[error(transparent)] GitOpen(#[from] gix::open::Error),
    #[error(transparent)] GitRevWalk(#[from] gix::revision::walk::Error),
    #[error(transparent)] GitRevWalkIter(#[from] gix::revision::walk::iter::Error),
    #[error("git commit timestamp out of range for chrono::DateTime")]
    TimeRange,
}

#[wheel::main]
fn main(Args { repo_path }: Args) -> Result<(), Error> {
    let mut repo = if let Some(repo_path) = repo_path {
        gix::open(repo_path)?
    } else {
        gix::discover(".")?
    };
    repo.object_cache_size_if_unset(1024 * 1024);
    let mut commits_by_date = BTreeMap::default();
    let head = repo.head_commit()?;
    let head_time = head.time()?;
    if head_time.is_set() {
        commits_by_date.insert(DateTime::from_timestamp(head_time.seconds, 0).ok_or(Error::TimeRange)?.date_naive(), 1);
    }
    for commit in head.ancestors().sorting(gix::revision::walk::Sorting::ByCommitTime(gix::traverse::commit::simple::CommitTimeOrder::default())).all()? {
        *commits_by_date.entry(DateTime::from_timestamp(commit?.commit_time(), 0).ok_or(Error::TimeRange)?.date_naive()).or_default() += 1;
    }
    let Some((first_date, _)) = commits_by_date.first_key_value() else { return Ok(()) };
    let Some((last_date, _)) = commits_by_date.last_key_value() else { return Ok(()) };
    let mut is_first_year = true;
    for year in first_date.year()..=last_date.year() {
        if is_first_year {
            is_first_year = false;
        } else {
            println!();
        }
        println!("{year}");
        for weekday in [Mon, Tue, Wed, Thu, Fri, Sat, Sun] {
            print!("{weekday:?} ");
            for week in 0..=53 {
                if_chain! {
                    if let Some(date) = if week == 0 { NaiveDate::from_isoywd_opt(year, 1, weekday).map(|date| date - chrono::Days::new(7)) } else { NaiveDate::from_isoywd_opt(year, week, weekday) };
                    if date.year() == year;
                    if let Some(&count) = commits_by_date.get(&date);
                    if count > 0;
                    then {
                        if count >= 10 {
                            print!("X");
                        } else {
                            print!("{count}");
                        }
                    } else {
                        print!(" ");
                    }
                }
            }
            println!();
        }
    }
    Ok(())
}
