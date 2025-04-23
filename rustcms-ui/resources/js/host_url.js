try {
    dioxus.send(window.location.origin);
} catch (error) {
    console.log(error);
    dioxus.send(window.location.href);
}