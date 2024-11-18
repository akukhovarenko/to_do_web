if (localStorage.getItem("user-token") == null) {
    window.location.replace(
        document.location.origin + '/api/v1/login'
    );
}

function renderItems(items, processType, elementId, processFunction) {
    let placeholder = "<div>";
    let itemsMeta = [];

    for (i = 0; i < items.length; i++) {
        let title = items[i]["title"];
        let placeholderId = processType + "-" + title.replace(" ", "-");
        placeholder += '<div class="itemContainer"><p>' + title + '</p><div class="actionButton" ' + 'id="' + placeholderId + '">' + processType + "</div></div>";
        itemsMeta.push({ "id": placeholderId, "title": title });
    }
    placeholder += "</div>"
    document.getElementById(elementId).innerHTML = placeholder;
    for (i = 0; i < itemsMeta.length; i++) {
        document.getElementById(itemsMeta[i]["id"]).addEventListener("click", processFunction)
    }
}

function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function () {
        if (this.readyState == this.LOADING) {
            console.log( Date.now() + "Show loading gif");
        }
        if (this.readyState == this.DONE) {
            renderItems(JSON.parse(this.responseText)["pending_items"], "Edit", "pendingItems", editItem);
            renderItems(JSON.parse(this.responseText)["done_items"], "Delete", "doneItems", deleteItem);
            document.getElementById("completeNum").innerHTML = JSON.parse(this.responseText)["done_items_count"];
            document.getElementById("pendingNum").innerHTML = JSON.parse(this.responseText)["pending_items_count"];
        }
    });
    xhr.open(method, "/api/v1" + url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', localStorage.getItem("user-token"));
    return xhr
}

function editItem() {
    let title = this.id.replaceAll("-", " ").replace("Edit ", "");
    let call = apiCall("/item/edit", "PUT");
    let json = { "title": title, "status": "done" };
    call.send(JSON.stringify(json));
}

function deleteItem() {
    let title = this.id.replaceAll("-", " ").replace("Delete ", "");
    let call = apiCall("/item/delete", "POST");
    let json = { "title": title, "status": "done" };
    call.send(JSON.stringify(json));
}

function getItems() {
    let call = apiCall("/item/get", "GET");
    call.send();
}

getItems();
document.getElementById("create-button").addEventListener("click", createItem);

function createItem() {
    let title = document.getElementById("name");
    let call = apiCall("/item/create/" + title.value, "POST");
    call.send();
    document.getElementById("name").value = null;

}
