;;; export_ics.el --- Export Org files to iCalendar with HTML links

(require 'ox-icalendar)

(defun my-icalendar-link-transcoder (link desc info)
  "Transcode a LINK object into HTML format for iCalendar DESCRIPTION."
  (let ((path (org-element-property :raw-link link)))
    (if (string-match-p "^https?://" path)
        (if desc
            (format "<a href=\"%s\">%s</a>" path desc)
          (format "<a href=\"%s\">%s</a>" path path))
      (if desc (format "[%s] (%s)" desc path) path))))

(defun my-icalendar-paragraph-transcoder (paragraph contents info)
  "Transcode a PARAGRAPH element into iCalendar format."
  ;; Replace internal newlines with spaces to allow natural wrapping
  ;; But double newlines (paragraph breaks) should be preserved if passed as such,
  ;; though usually 'contents' here is just the inner text.
  ;; We trim surrounding whitespace and replace single newlines with spaces.
  (let ((text (org-trim contents)))
    (replace-regexp-in-string "\n" " " text)))

(org-export-define-derived-backend 'icalendar-html-links 'icalendar
  :translate-alist '((link . my-icalendar-link-transcoder)
                     (paragraph . my-icalendar-paragraph-transcoder)))

(defun crewd/export-calendars ()
  "Export project calendar files using the custom backend."
  (let ((files '("data/series.org" "data/schedule.org"))
        (root-dir default-directory))
    (dolist (f files)
      (let ((full-path (expand-file-name f root-dir)))
        (if (file-exists-p full-path)
            (with-current-buffer (find-file-noselect full-path)
              (let ((org-icalendar-store-link nil)
                    (org-icalendar-include-body t))
                (org-export-to-file 'icalendar-html-links
                  (concat (file-name-sans-extension full-path) ".ics"))
                (message "Exported %s" full-path)))
          (message "File not found: %s" full-path))))))

(crewd/export-calendars)
