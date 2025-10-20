use crate::domain::{
    CreateTodoItemRequest, Error, Filters, TodoCounter, TodoCreator, TodoGetter, TodoItem,
    TodoLister, TodoListerAndCounter, TodoUpdater, UpdateTodoItemRequest,
};

pub struct TodoService<CR, CO, LI, GE, UP>
where
    CR: TodoCreator,
    CO: TodoCounter,
    LI: TodoLister,
    GE: TodoGetter,
    UP: TodoUpdater,
{
    creator: CR,
    counter: CO,
    lister: LI,
    getter: GE,
    updater: UP,
}

impl<CR, CO, LI, GE, UP> TodoService<CR, CO, LI, GE, UP>
where
    CR: TodoCreator,
    CO: TodoCounter,
    LI: TodoLister,
    GE: TodoGetter,
    UP: TodoUpdater,
{
    pub fn new(creator: CR, counter: CO, lister: LI, getter: GE, updater: UP) -> Self {
        Self {
            creator: creator,
            counter: counter,
            lister: lister,
            getter: getter,
            updater: updater,
        }
    }
}

#[async_trait]
impl<CR, CO, LI, GE, UP> TodoCreator for TodoService<CR, CO, LI, GE, UP>
where
    CR: TodoCreator,
    CO: TodoCounter,
    LI: TodoLister,
    GE: TodoGetter,
    UP: TodoUpdater,
{
    async fn create(&self, request: CreateTodoItemRequest) -> Result<TodoItem, Error> {
        self.creator.create(request).await
    }
}

#[async_trait]
impl<CR, CO, LI, GE, UP> TodoListerAndCounter for TodoService<CR, CO, LI, GE, UP>
where
    CR: TodoCreator,
    CO: TodoCounter,
    LI: TodoLister,
    GE: TodoGetter,
    UP: TodoUpdater,
{
    async fn list(&self, filters: &Filters) -> Result<(Vec<TodoItem>, i64), Error> {
        let total = self.counter.count(filters).await?;
        let list = self.lister.list(filters).await?;
        Ok((list, total))
    }
}

#[async_trait]
impl<CR, CO, LI, GE, UP> TodoGetter for TodoService<CR, CO, LI, GE, UP>
where
    CR: TodoCreator,
    CO: TodoCounter,
    LI: TodoLister,
    GE: TodoGetter,
    UP: TodoUpdater,
{
    async fn one(&self, id: i32) -> Result<TodoItem, Error> {
        self.getter.one(id).await
    }
}
#[async_trait]
impl<CR, CO, LI, GE, UP> TodoUpdater for TodoService<CR, CO, LI, GE, UP>
where
    CR: TodoCreator,
    CO: TodoCounter,
    LI: TodoLister,
    GE: TodoGetter,
    UP: TodoUpdater,
{
    async fn update(&self, request: UpdateTodoItemRequest) -> Result<TodoItem, Error> {
        let stored = self.getter.one(request.item_id).await?;
        if stored.owner_id != request.owner_id {
            return Err(Error::Forbidden(format!(
                "owner is {}, but received request from {}",
                stored.owner_id, request.owner_id,
            )));
        }

        if !stored.status.can_be_updated_to(&request.status) {
            return Err(Error::OperationNotApplicable(format!(
                "can't update from {} to {}",
                stored.status.to_string(),
                request.status.to_string()
            )));
        }

        self.updater.update(request).await
    }
}
