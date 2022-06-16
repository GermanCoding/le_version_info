for (const endpoint of ["prod", "staging"]) {
    $.getJSON('./' + endpoint + '.json', function (data) {
        let table = $("#" + endpoint + "-body");
        let body_row = [];

        $.each(data.reverse(), function (index, item) {
            body_row[index] = '<tr><td class="text-start">' + item.build + '</td><td class="text-start">' + item.first_seen + '</td><td class="text-start">' + (item.last_seen ? item.last_seen : 'now') + '</td></tr>';
        });

        table.append(body_row);
        $("#" + endpoint).DataTable({"ordering": false});
    });
}
