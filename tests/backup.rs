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

use crate::util::setup;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[tokio::test]
async fn test_backup() -> Result<(), Box<dyn std::error::Error>> {
    let tm = setup().await?;

    tm.command()?
        .args(["-r", "local", "backup"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "To execute the command you must specify target table in one of following ways:",
        ));

    tm.command()?
        .args(["-r", "local", "backup", "--table", "non-existent-table"])
        .assert()
        .failure()
        .stdout(predicate::str::contains(
            // This error message only happens on DynamoDB Local which does not support backup feature.
            "com.amazonaws.dynamodb.v20120810#UnknownOperationException",
        ));

    Ok(())
}