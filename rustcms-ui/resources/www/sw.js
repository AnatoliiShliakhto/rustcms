const CACHE_VERSION = '0.1.0'
const CACHE_NAME = `mtc-cache-${CACHE_VERSION}`
const PRE_CACHED_RESOURCES = ['index.html']

let accessToken = null

self.addEventListener('install', (event) => event.waitUntil(addResourcesToCache(PRE_CACHED_RESOURCES)));
self.addEventListener('activate', (event) => {
    event.waitUntil(self.registration?.navigationPreload.enable())
    event.waitUntil(self.clients.claim())
    event.waitUntil(caches.keys().then((keys) => {
        return Promise.all(keys.filter((key) => key !== CACHE_NAME)
            .map((nm) => caches.delete(nm)))
    }))
    event.waitUntil(addResourcesToCache(PRE_CACHED_RESOURCES))
})
self.addEventListener('message', async (event) => {
    if (event?.data?.type === 'ACTIVATE') {
        self.skipWaiting().then(() => {
            console.log('ServiceWorker reactivated')
        })
    } else if (event?.data?.type === 'VERSION') {
        event.ports[0].postMessage({
            type: 'VERSION', version: CACHE_VERSION,
        })
    } else if (event?.data?.type === 'CLEAR_CACHE') {
        try {
            caches.keys().then((keys) => {
                keys.map((key) => caches.delete(key)
                    .then(() => caches.open(CACHE_NAME)
                        .then((cache) => cache.addAll(PRE_CACHED_RESOURCES))
                        .then(() => {
                            event.ports[0].postMessage({
                                type: 'CLEAR_CACHE', result: true,
                            })
                        })))
            })
        } catch (error) {
            event.ports[0].postMessage({
                type: 'CLEAR_CACHE', result: false,
            })
        }
    }
})
self.addEventListener('fetch', async (event) => {
    const {request} = event
    const {url} = request

    if (url.startsWith(`${self.location.origin}/api/v1/auth`)) {
        if (request.method === 'GET') {
            event.respondWith(signIn(request))
        } else if (request.method === 'DELETE') {
            event.respondWith(signOut(request))
        }
    } else if (url.startsWith(`${self.location.origin}/api`) || url.startsWith(`${self.location.origin}/private`)) {
        event.respondWith(fetchWithToken(request))
    }
})

const addResourcesToCache = async (resources) => {
    caches.open(CACHE_NAME).then((cache) => cache.addAll(resources));
};

const signIn = async (request) => {
    const response = await fetch(request, {credentials: 'include'})

    const token = await response.clone().json().then((data) => {
        return data.access_token
    })

    if (token) {
        accessToken = token

        const {headers, status, statusText} = response.clone()

        return new Response(null, {
            headers,
            status,
            statusText
        })
    }

    return response
}

const signOut = async (request) => {
    return await fetch(request, {
        method: 'DELETE',
        credentials: 'include'
    }).then((response) => {
        accessToken = null
        return response
    })
}

const fetchWithToken = async (request) => {
    let response = await fetch(request.clone(), {
        headers: {
            Authorization: `Bearer ${accessToken}`
        }
    })

    if (response.status === 401) {
        await refreshAccessToken()

        response = await fetch(request, {
            headers: {
                Authorization: `Bearer ${accessToken}`
            }
        })
    }

    return response
}

const refreshAccessToken = async () => {
    const response = await fetch(`${self.location.origin}/api/v1/auth/token`, {
        method: 'GET',
        credentials: 'include'
    })

    const token = await response.json().then((data) => {
        return data.access_token
    })

    if (token) {
        accessToken = token
    }
}