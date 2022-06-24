for (const endpoint of ["prod", "staging"]) {
    $.getJSON('./' + endpoint + '.json', function (data) {
        let table = $("#" + endpoint + "-body");
        let body_row = [];
        let rdata = data.reverse();

        $.each(rdata, function (index, item) {
            let commit = null;
            let prev_commit = null;
            if (item.build != null && item.build.startsWith("+")) {
                commit = item.build.substring(1, item.build.indexOf(" "));
            }
            if (rdata.length > index + 1) {
                let prev_item = rdata[index + 1];
                if (prev_item.build != null && prev_item.build.startsWith("+")) {
                    prev_commit = prev_item.build.substring(1, prev_item.build.indexOf(" "));
                }
            }

            if (commit != null && prev_commit != null) {
                body_row[index] = '<tr><td class="text-start"><a href=\"https://github.com/letsencrypt/boulder/compare/' + prev_commit + "..." + commit + "\">" + item.build + '</a></td>';
            } else if (commit != null) {
                body_row[index] = '<tr><td class="text-start"><a href=\"https://github.com/letsencrypt/boulder/commit/' + commit + "\">" + item.build + '</a></td>';
            } else {
                body_row[index] = '<tr><td class="text-start">' + item.build + '</td>';
            }
            body_row[index] += '<td class="text-start">' + item.first_seen + '</td><td class="text-start">' + (item.last_seen ? item.last_seen : 'now') + '</td></tr>';
        });

        table.append(body_row);
        $("#" + endpoint).DataTable({"ordering": false});
    });
}
