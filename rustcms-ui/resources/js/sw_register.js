if ('serviceWorker' in navigator) {
    navigator
        .serviceWorker
        .register('./sw.js', {scope: './', updateViaCache: 'none'})
        .then((registration) => {
            if (self.intervalId) clearInterval(self.intervalId)

            self.intervalId = setInterval(async () => {
                await registration.update()
            }, 5 * 60 * 1000)

            registration.addEventListener('updateFound', () => {
                registration?.waiting?.postMessage({type: 'ACTIVATE'})
            })
        })
        .catch((error) => console.log('ServiceWorker registration failed', error))
}