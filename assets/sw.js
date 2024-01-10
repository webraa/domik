var cacheName = 'domik-pwa';
var filesToCache = [
  './',
  './index.html',
  './domik.js',
  './domik_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  console.log('--> raa 3')
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      console.log('--> raa 3.1')
      return cache.addAll(filesToCache);
    })
  );
});


/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  console.log('--> raa 4')
  e.respondWith(
    caches.match(e.request).then(function (response) {
      console.log('--> raa 4.1')
      return response || fetch(e.request);
    })
  );
});

