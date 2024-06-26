/*
* Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
*
* Licensed under the Apache License, Version 2.0 (the "License").
* You may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
pub mod util;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[tokio::test]
async fn test_list_table_with_no_table() -> Result<(), Box<dyn std::error::Error>> {
    let tm = util::setup_with_lock().await?;

    let mut c = tm.command()?;
    let cmd = c.args(["--region", "local", "list"]);
    cmd.assert().success().stdout(
        predicate::str::is_match(
            "DynamoDB tables in region: local
  No table in this region.",
        )
        .unwrap(),
    );

    Ok(())
}

#[tokio::test]
async fn test_list_table_with_multiple_tables() -> Result<(), Box<dyn std::error::Error>> {
    let mut tm = util::setup_with_lock().await?;
    let table_name = tm.create_temporary_table("pk", None).await?;
    let table_name2 = tm.create_temporary_table("pk", None).await?;

    let mut c = tm.command()?;
    let cmd = c.args(["--region", "local", "use", &table_name]);
    cmd.assert().success();

    let mut c = tm.command()?;
    let cmd = c.args(["--region", "local", "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DynamoDB tables in region: local"))
        .stdout(predicate::str::contains(format!("* {table_name}")))
        .stdout(predicate::str::contains(table_name2));
    Ok(())
}

#[tokio::test]
async fn test_list_table_ls_alias() -> Result<(), Box<dyn std::error::Error>> {
    let mut tm = util::setup_with_lock().await?;
    let table_name = tm.create_temporary_table("pk", None).await?;

    let mut c = tm.command()?;
    let cmd = c.args(["--region", "local", "use", &table_name]);
    cmd.assert().success();

    let mut c = tm.command()?;
    let cmd = c.args(["--region", "local", "ls"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DynamoDB tables in region: local"))
        .stdout(predicate::str::contains(format!("* {table_name}")));
    Ok(())
}
