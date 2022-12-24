use cursive::{views::{LinearLayout, DummyView, NamedView}, view::{ViewWrapper, Nameable, Resizable}};
use super::{views::{TasksView, ProjectsView, InfoView, ActionsView}, constance::INFO_LAYOUT_VIEW_NAME};


pub(crate) struct TasksProjectsLayout {
    inner_layout: LinearLayout,
}

impl Default for TasksProjectsLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(ProjectsView::default().full_height())
                .child(DummyView)
                .child(TasksView::default().full_height())
        }
    }
}

impl ViewWrapper for TasksProjectsLayout {
    type V = LinearLayout;

    fn with_view<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&Self::V) -> R {
        return Some(f(&self.inner_layout));
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R> where F: FnOnce(&mut Self::V) -> R {
        return Some(f(&mut self.inner_layout));
    }
}

pub(crate) struct InfoLayout {
    inner_layout: NamedView<LinearLayout>,
}

impl Default for InfoLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(InfoView::default())
                .with_name(INFO_LAYOUT_VIEW_NAME)
        }
    }
}

impl ViewWrapper for InfoLayout {
    type V = NamedView<LinearLayout>;

    fn with_view<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&Self::V) -> R {
        return Some(f(&self.inner_layout));
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R> where F: FnOnce(&mut Self::V) -> R {
        return Some(f(&mut self.inner_layout));
    }
}

pub(crate) struct ActionsLayout {
    inner_layout: LinearLayout,
}

impl Default for ActionsLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(ActionsView::default().full_height())
        }
    }
}

impl ViewWrapper for ActionsLayout {
    type V = LinearLayout;

    fn with_view<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&Self::V) -> R {
        return Some(f(&self.inner_layout));
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R> where F: FnOnce(&mut Self::V) -> R {
        return Some(f(&mut self.inner_layout));
    }
}
