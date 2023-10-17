use crate::model;
use crate::pages::{cve::result::packages::PackagesTable, search::PaginationWrapped};
use patternfly_yew::prelude::*;
use spog_model::prelude::{CveDetails, PackageRelatedToProductCve, ProductCveStatus};
use spog_ui_backend::{use_backend, SBOMService};
use spog_ui_components::async_state_renderer::async_content;
use spog_ui_navigation::{AppRoute, View};
use std::rc::Rc;
use yew::prelude::*;
use yew_more_hooks::prelude::use_async_with_cloned_deps;
use yew_nested_router::components::Link;
use yew_oauth2::prelude::use_latest_access_token;

#[derive(PartialEq)]
pub struct TableData {
    sbom: Option<Rc<model::SBOM>>,
    sbom_id: String,
    status: ProductCveStatus,
    packages: Vec<PackageRelatedToProductCve>,
}

trait StatusLabel {
    fn label(&self) -> &'static str;
    fn color(&self) -> Color;
}

impl StatusLabel for ProductCveStatus {
    fn label(&self) -> &'static str {
        match self {
            Self::Fixed => "Fixed",
            Self::FirstFixed => "First fixed",
            Self::FirstAffected => "First Affected",
            Self::KnownAffected => "Known affected",
            Self::LastAffected => "Last affected",
            Self::KnownNotAffected => "Knwon not affected",
            Self::Recommended => "Recommended",
            Self::UnderInvestigation => "Under investigation",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Fixed => Color::Green,
            Self::FirstFixed => Color::Green,
            Self::FirstAffected => Color::Red,
            Self::KnownAffected => Color::Red,
            Self::LastAffected => Color::Red,
            Self::KnownNotAffected => Color::Blue,
            Self::Recommended => Color::Orange,
            Self::UnderInvestigation => Color::Grey,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Column {
    Name,
    Version,
    Status,
    Dependencies,
    Supplier,
    ReleasedOn,
}

impl TableEntryRenderer<Column> for TableData {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            Column::Name => html!(
                <Link<AppRoute>
                    target={AppRoute::Sbom(View::Content{id: self.sbom_id.clone()})}
                >
                    { self.sbom_id.clone() }
                </Link<AppRoute>>
            ),
            Column::Version => html!(<></>),
            Column::Status => html!(
                <>
                    <Label label={self.status.label().to_string()} color={self.status.color()}/>
                </>
            ),
            Column::Dependencies => html!(
                <>
                    {&self.packages.len()}
                </>
            ),
            Column::Supplier => html!(<></>),
            Column::ReleasedOn => html!(<></>),
        }
        .into()
    }

    fn render_column_details(&self, #[allow(unused)] column: &Column) -> Vec<Span> {
        vec![Span::max(match column {
            Column::Name => html!({ "Name" }),
            Column::Version => html!({ "Version" }),
            Column::Status => html!({ "Status" }),
            Column::Dependencies => html!(<PackagesTable packages={self.packages.clone()} />),
            Column::Supplier => html!({ "Supplier" }),
            Column::ReleasedOn => html!({ "ReleasedOn" }),
        })]
    }
}

#[derive(PartialEq, Properties)]
pub struct RelatedProductsProperties {
    pub cve_details: Rc<CveDetails>,
}

#[function_component(RelatedProducts)]
pub fn related_products(props: &RelatedProductsProperties) -> Html {
    let backend = use_backend();
    let access_token = use_latest_access_token();

    let table_data = use_memo(props.cve_details.products.clone(), |map| {
        map.iter()
            .flat_map(|(map_key, map_value)| {
                map_value
                    .iter()
                    .map(|item| TableData {
                        status: map_key.clone(),
                        packages: item.packages.clone(),
                        sbom_id: item.sbom_id.to_string(),
                        sbom: None,
                    })
                    .collect::<Vec<TableData>>()
            })
            .collect::<Vec<TableData>>()
    });

    let table_data_sboms = {
        let backend = backend.clone();
        let access_token = access_token.clone();
        use_async_with_cloned_deps(
            |table_data| async move {
                let sbom_ids = table_data.iter().map(|e| e.sbom_id.clone()).collect::<Vec<String>>();
                let service = SBOMService::new(backend.clone(), access_token.clone());
                service
                    .get_batch(&sbom_ids)
                    .await
                    .map(|response| {
                        let r = response
                            .iter()
                            .map(|elem| elem.clone().map(model::SBOM::parse).map(Rc::new))
                            .collect::<Vec<_>>();
                        Rc::new(r)
                    })
                    .map_err(|err| err.to_string())
            },
            table_data.clone(),
        )
    };

    match props.cve_details.products.is_empty() {
        true => html!(
            <Panel>
                <PanelMain>
                    <Bullseye>
                        <EmptyState
                            title="No related products"
                            icon={Icon::Search}
                        >
                            { "No related products have been found." }
                        </EmptyState>
                    </Bullseye>
                </PanelMain>
            </Panel>
        ),
        false => html!(
            <>
                { async_content(&*table_data_sboms, |sboms| html!(<RelatedProductsResult table_data={table_data} table_data_sboms={sboms}/>)) }
            </>
        ),
    }
}

//

#[derive(PartialEq, Properties)]
pub struct RelatedProductsResultProperties {
    pub table_data: Rc<Vec<TableData>>,
    pub table_data_sboms: Rc<Vec<Option<Rc<model::SBOM>>>>,
}

#[function_component(RelatedProductsResult)]
pub fn related_products_result(props: &RelatedProductsResultProperties) -> Html {
    let table_data = use_memo(
        (props.table_data.clone(), props.table_data_sboms.clone()),
        |(table_data, table_data_sboms)| {
            table_data
                .iter()
                .enumerate()
                .map(|(index, elem)| {
                    let sbom = &table_data_sboms[index];
                    TableData {
                        sbom: sbom.clone(),
                        status: elem.status.clone(),
                        packages: elem.packages.clone(),
                        sbom_id: elem.sbom_id.to_string(),
                    }
                })
                .collect::<Vec<TableData>>()
        },
    );

    let total = table_data.len();
    let (entries, onexpand) = use_table_data(MemoizedTableModel::new(table_data));

    let header = html_nested! {
        <TableHeader<Column>>
            <TableColumn<Column> label="Name" index={Column::Name} />
            <TableColumn<Column> label="Version" index={Column::Version} />
            <TableColumn<Column> label="Status" index={Column::Status} />
            <TableColumn<Column> label="Dependencies" index={Column::Dependencies} expandable=true />
            <TableColumn<Column> label="Supplier" index={Column::Supplier} />
            <TableColumn<Column> label="Released on" index={Column::ReleasedOn} />
        </TableHeader<Column>>
    };

    let pagination = use_pagination(Some(total), || PaginationControl { page: 1, per_page: 10 });

    html!(
        <div class="pf-v5-u-background-color-100">
            <PaginationWrapped pagination={pagination} total={10}>
                <Table<Column, UseTableData<Column, MemoizedTableModel<TableData>>>
                    mode={TableMode::Expandable}
                    {header}
                    {entries}
                    {onexpand}
                />
            </PaginationWrapped>
        </div>
    )
}
