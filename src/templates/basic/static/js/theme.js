// Minimal theme switcher for GitHub-themed blog
(function() {
    'use strict';

    // Theme management
    const theme = {
        // Get current theme
        get() {
            return document.documentElement.getAttribute('data-theme') || 'dark';
        },

        // Set theme
        set(themeName) {
            document.documentElement.setAttribute('data-theme', themeName);
            localStorage.setItem('theme', themeName);
        },

        // Toggle between light and dark
        toggle() {
            const current = this.get();
            const next = current === 'dark' ? 'light' : 'dark';

            // Add transition class for smooth change
            document.body.classList.add('theme-transition');

            this.set(next);

            // Remove transition class after animation
            setTimeout(() => {
                document.body.classList.remove('theme-transition');
            }, 300);
        },

        // Initialize theme
        init() {
            // Check for saved preference or system preference
            const saved = localStorage.getItem('theme');
            const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            const defaultTheme = systemPrefersDark ? 'dark' : 'light';

            this.set(saved || defaultTheme);

            // Listen for system theme changes
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
                if (!localStorage.getItem('theme')) {
                    this.set(e.matches ? 'dark' : 'light');
                }
            });
        }
    };

    // Initialize theme
    theme.init();

    // Setup theme toggle button
    const setupThemeToggle = () => {
        const toggleBtn = document.querySelector('.theme-toggle');
        if (toggleBtn) {
            toggleBtn.addEventListener('click', () => theme.toggle());
        }
    };

    // Mobile navigation
    const setupMobileNav = () => {
        const toggle = document.querySelector('.nav-toggle');
        const menu = document.querySelector('.nav-menu');

        if (!toggle || !menu) return;

        // Toggle menu
        toggle.addEventListener('click', () => {
            const isOpen = toggle.getAttribute('aria-expanded') === 'true';
            toggle.setAttribute('aria-expanded', !isOpen);
            menu.classList.toggle('is-open');
        });

        // Close menu when clicking outside
        document.addEventListener('click', (e) => {
            if (!toggle.contains(e.target) && !menu.contains(e.target)) {
                toggle.setAttribute('aria-expanded', 'false');
                menu.classList.remove('is-open');
            }
        });

        // Close menu on escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && menu.classList.contains('is-open')) {
                toggle.setAttribute('aria-expanded', 'false');
                menu.classList.remove('is-open');
                toggle.focus();
            }
        });
    };

    // Smooth scroll for anchor links
    const setupSmoothScroll = () => {
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', (e) => {
                e.preventDefault();
                const target = document.querySelector(anchor.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({ behavior: 'smooth', block: 'start' });
                }
            });
        });
    };

    // Back to top button (if exists)
    const setupBackToTop = () => {
        const btn = document.querySelector('.back-to-top');
        if (!btn) return;

        // Show/hide based on scroll position
        let lastScroll = 0;
        window.addEventListener('scroll', () => {
            const currentScroll = window.pageYOffset;

            if (currentScroll > 300) {
                btn.classList.add('visible');
            } else {
                btn.classList.remove('visible');
            }

            lastScroll = currentScroll;
        }, { passive: true });

        // Scroll to top on click
        btn.addEventListener('click', () => {
            window.scrollTo({ top: 0, behavior: 'smooth' });
        });
    };

    // Reading progress indicator (if exists)
    const setupReadingProgress = () => {
        const progress = document.querySelector('.reading-progress');
        if (!progress) return;

        window.addEventListener('scroll', () => {
            const winScroll = document.body.scrollTop || document.documentElement.scrollTop;
            const height = document.documentElement.scrollHeight - document.documentElement.clientHeight;
            const scrolled = (winScroll / height) * 100;
            progress.style.width = scrolled + '%';
        }, { passive: true });
    };

    // Copy code blocks to clipboard
    const setupCodeCopy = () => {
        document.querySelectorAll('pre').forEach(pre => {
            // Create copy button
            const btn = document.createElement('button');
            btn.className = 'code-copy';
            btn.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>';
            btn.setAttribute('aria-label', 'Copy code');

            // Copy functionality
            btn.addEventListener('click', async () => {
                const code = pre.textContent;
                try {
                    await navigator.clipboard.writeText(code);
                    btn.classList.add('copied');
                    setTimeout(() => btn.classList.remove('copied'), 2000);
                } catch (err) {
                    console.error('Failed to copy:', err);
                }
            });

            // Add button to pre element
            pre.style.position = 'relative';
            pre.appendChild(btn);
        });
    };

    // Initialize all features when DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            setupThemeToggle();
            setupMobileNav();
            setupSmoothScroll();
            setupBackToTop();
            setupReadingProgress();
            setupCodeCopy();
        });
    } else {
        setupThemeToggle();
        setupMobileNav();
        setupSmoothScroll();
        setupBackToTop();
        setupReadingProgress();
        setupCodeCopy();
    }
})();
