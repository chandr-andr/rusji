use cursive::{views::{LinearLayout, DummyView, NamedView, ViewRef}, view::{ViewWrapper, Nameable, Resizable, Finder}, Cursive};
use super::{views::{TasksView, ProjectsView, InfoView, ActionsView, JiraView}};


pub(crate) struct TasksProjectsLayout {
    inner_layout: LinearLayout,
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

impl Default for TasksProjectsLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(
                    ProjectsView::default()
                        .with_name(ProjectsView::view_name())
                        .full_height()
                )
                .child(DummyView)
                .child(
                    TasksView::default()
                        .with_name(TasksView::view_name())
                        .full_height())
        }
    }
}

pub(crate) struct InfoLayout {
    inner_layout: NamedView<LinearLayout>,
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

impl Default for InfoLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(
                    InfoView::default()
                        .with_name(InfoView::view_name()),
                )
                .with_name(Self::inner_layout_name())
        }
    }
}

impl InfoLayout {
    pub fn layout_name() -> String {
        String::from("InfoLayout")
    }

    pub fn inner_layout_name() -> String {
        String::from("InnerInfoLayout")
    }

    pub fn get_layout(cursive: &mut Cursive) -> ViewRef<Self> {
        cursive.find_name(&Self::layout_name()).unwrap()
    }

    pub fn get_inner_layout(&mut self) -> ViewRef<LinearLayout> {
        self.find_name(&Self::inner_layout_name()).unwrap()
    }
}

pub(crate) struct ActionsLayout {
    inner_layout: LinearLayout,
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

impl Default for ActionsLayout {
    fn default() -> Self {
        Self {
            inner_layout: LinearLayout::vertical()
                .child(ActionsView::default().full_height())
        }
    }
}
